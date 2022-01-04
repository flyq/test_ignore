import Principal "mo:base/Principal";

actor {
    stable var a: Text = "";

    type TestActor = actor {
        set: (a: Text) -> async Bool;
    };

    public func set(_a: Text): async Bool {
        a := _a;
        true
    };

    public func call(_a: Text, to: Principal): async Bool {
        let handle: TestActor = actor(Principal.toText(to));
        // ignore handle.set(_a); works
        // let _ = handle.set(_a); works
        // true
        let result = await handle.set(_a);
        result
    };

    public query func get(): async Text {
        a
    };
};