@Structure.FieldOrder("code", "message")
internal open class {{e.name()}} : Structure() {

    class ByReference : {{e.name()}}(), Structure.ByReference

    @JvmField var code: Int = 0
    @JvmField var message: Pointer? = null

    /**
     * Does this represent success?
     */
    fun isSuccess(): Boolean {
        return code == 0
    }

    /**
     * Does this represent failure?
     */
    fun isFailure(): Boolean {
        return code != 0
    }

    @Suppress("ReturnCount", "TooGenericExceptionThrown")
    fun intoException(): {{e.name()}}Exception {
        if (!isFailure()) {
            // It's probably a bad idea to throw here! We're probably leaking something if this is
            // ever hit! (But we shouldn't ever hit it?)
            throw RuntimeException("[Bug] intoException called on non-failure!")
        }
        val message = this.consumeErrorMessage()
        when (code) {
            {% for value in e.values() -%}
            {{loop.index}} -> return {{e.name()}}Exception.{{value}}(message)
            {% endfor -%}
            else -> throw RuntimeException("Invalid error recieved...")
        }
    }

    /**
     * Get and consume the error message, or null if there is none.
     */
    @Synchronized
    fun consumeErrorMessage(): String {
        val result = this.getMessage()
        if (this.message != null) {
            this.ensureConsumed()
        }
        if (result == null) {
            throw NullPointerException("consumeErrorMessage called with null message!")
        }
        return result
    }

    @Synchronized
    fun ensureConsumed() {
        if (this.message != null) {
            _UniFFILib.INSTANCE.{{ci.namespace()}}_string_free(this.message!!)
            this.message = null
        }
    }

    /**
     * Get the error message or null if there is none.
     */
    fun getMessage(): String? {
        return this.message?.getString(0, "utf8")
    }
}

open class {{e.name()}}Exception(message: String) : Exception(message) {
    {% for value in e.values() -%}
    class {{value}}(msg: String) : {{e.name()}}Exception(msg)
    {% endfor %}
}

// Helpers for calling Rust with errors:
// In practice we usually need to be synchronized to call this safely, so it doesn't
// synchronize itself
private inline fun <U> nullableRustCallWith{{e.name()}}(callback: ({{e.name()}}.ByReference) -> U?): U? {
    val e = {{e.name()}}.ByReference()
    try {
        val ret = callback(e)
        if (e.isFailure()) {
            throw e.intoException()
        }
        return ret
    } finally {
        // This only matters if `callback` throws (or does a non-local return, which
        // we currently don't do)
        e.ensureConsumed()
    }
}

private inline fun <U> rustCallWith{{e.name()}}(callback: ({{e.name()}}.ByReference) -> U?): U {
    return nullableRustCallWith{{e.name()}}(callback)!!
}
