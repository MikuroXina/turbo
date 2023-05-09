export interface Param {
    type: symbol;
}

const boolean = Symbol("BOOLEAN");
export class Boolean implements Param {
    readonly type = boolean;

    readonly value: boolean = false;

    readonly defaultValue: boolean;

    readonly radioButtonGroup: string = "";

    constructor(b: boolean, radioButtonGroup: string) {
        this.value = b;
        this.defaultValue = b;
        this.radioButtonGroup = radioButtonGroup;
    }
}

const vec2 = Symbol("VEC2");
export class Vec2 implements Param {
    readonly type = vec2;

    readonly value: readonly [number, number] = [0, 0];

    readonly defaultValue: readonly [number, number];

    constructor(a = 0.0, b = 0.0) {
        this.value = [a, b];
        this.defaultValue = [a, b];
    }

    add(other: Vec2): Vec2 {
        throw new Error("todo");
    }

    sub(other: Vec2): Vec2 {
        throw new Error("todo");
    }
}
