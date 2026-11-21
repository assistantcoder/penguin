## All to do


## side todo

fix variables assign system and link it with AllowAssign--done
link the last AST with variable system in codegen--done--not sure if cranlift copy the values or the optimazition usauly v0 in:
function u0:0() -> i64 system_v {
block0:
    v0 = iconst.i64 10
    return v0  ; v0 = 10
}

## Next step is define calct ast, link it with allow assign but do not and more matcher just small function integreted, start from syntax matcher because will use it in many places

## Agreed decisions

### Development order
- Define Calc AST and connect it with AllowAssign.
- Add comparison expressions and operators.
- Add Bool support for comparison results and conditions.
- Implement if/else conditions using Bool.
- Implement loops after conditional flow works.

### String is deferred completely
- Remove String from the language core, AST, syntax, matcher, and codegen for now.
- Do not keep placeholder String nodes or partial support.
- Reintroduce String after implementing a dedicated mechanism for binding core-language functions to Cranelift, responsible for allocation.
- Keep String's design independent of direct Cranelift implementation details.

### Project pause
- Penguin development is paused until the winter break.
- Resume using the development order above.
