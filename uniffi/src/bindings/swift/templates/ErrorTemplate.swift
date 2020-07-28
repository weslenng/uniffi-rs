
public enum {{e.name()}}: LocalizedError {
    {% for value in e.values() %}
    case {{value}}(message: String)
    {% endfor %}


    /// Our implementation of the localizedError protocol
    public var errorDescription: String? {
        switch self {
        {% for value in e.values() %}
        case let .{{value}}(message):
            return "{{e.name()}}.{{value}}: \(message)"
        {% endfor %}
        }
    }

    // The name is attempting to indicate that we free message if it
    // existed, and that it's a very bad idea to touch it after you call this
    // function
    static func fromConsuming(_ rustError: Native{{e.name()}}) throws -> {{e.name()}}? {
        let message = rustError.message
        switch rustError.code {
            case {{e.name()}}_NoError:
                return nil
            {% for value in e.values() %}
            case {{e.name()}}_{{value}}:
                return .{{value}}(message: try String.fromFFIValue(message!))
            {% endfor %}
            default:
                return nil
        }
    }

    @discardableResult
    public static func unwrap<T>(_ callback: (UnsafeMutablePointer<Native{{e.name()}}>) throws -> T?) throws -> T {
        guard let result = try tryUnwrap(callback) else {
            throw {{e.name()}}ResultError.empty
        }
        return result
    }

    @discardableResult
    public static func tryUnwrap<T>(_ callback: (UnsafeMutablePointer<Native{{e.name()}}>) throws -> T?) throws -> T? {
        var err = Native{{e.name()}}(code: {{e.name()}}_NoError, message: nil)
        let returnedVal = try callback(&err)
        if let retErr = try {{e.name()}}.fromConsuming(err) {
            throw retErr
        }
        return returnedVal
    }
}

internal func rustCallWith{{e.name()}}<T>(_ cb: (UnsafeMutablePointer<Native{{e.name()}}>) throws -> T?) throws -> T {
    return try {{e.name()}}.unwrap { err in
        return try cb(err)
    }
}

internal func nullableRustCallWith{{e.name()}}<T>(_ cb: (UnsafeMutablePointer<Native{{e.name()}}>) throws -> T?) throws -> T? {
    return try {{e.name()}}.tryUnwrap { err in
        return try cb(err)
    }
}

public enum {{e.name()}}ResultError: Error {
    case empty
}