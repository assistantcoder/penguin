## All to do


## side todo

fix variables assign system and link it with AllowAssign--done
link the last AST with variable system in codegen--done--not sure if cranlift copy the values or the optimazition usauly v0 in:
function u0:0() -> i64 system_v {
block0:
    v0 = iconst.i64 10
    return v0  ; v0 = 10
}

### Development order
- Define Calc AST and connect it with AllowAssign.
- Add comparison expressions and operators.
- Add Bool support for comparison results and conditions --done
- Implement if/else conditions using Bool.
- Implement loops after conditional flow works.

### String is deferred completely --done
### Project pause
- Penguin development is paused until the winter break.
- Resume using the development order above.
