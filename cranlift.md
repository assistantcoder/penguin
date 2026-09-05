# دورة Cranelift من الصفر — الأساسيات

هنمشي خطوة خطوة. كل خطوة فيها: **المفهوم → المثال → ليش موجود**.

---

## الخطوة 1: الفكرة الكلية قبل أي كود

Cranelift يشتغل بأربع طبقات، بالترتيب:

```
1. Module        → "الحاوية" اللي فيها كل دوالك وبياناتك
2. Signature     → توقيع كل دالة (parameters + return type)
3. Function      → جسم الدالة (مبني من blocks)
4. Block         → تسلسل تعليمات ينتهي دايماً بقفزة/return
```

فكّرها كذا: **Module = ملف كامل، Function = دالة جوا الملف، Block = فقرة كود جوا الدالة**.

---

## الخطوة 2: أول شي — Module

قبل ما تكتب أي دالة، تحتاج "حاوية" تسجل فيها كل شي:

```rust
use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_module::default_libcall_names;
use target_lexicon::Triple;
use cranelift_codegen::settings;

let isa_builder = cranelift_codegen::isa::lookup(Triple::host()).unwrap();
let isa = isa_builder.finish(settings::Flags::new(settings::builder())).unwrap();
let obj_builder = ObjectBuilder::new(isa, "prog", default_libcall_names()).unwrap();
let mut module = ObjectModule::new(obj_builder);
```

**ليش هذا الطول؟** لازم تحدد "لأي معمارية أترجم" (`isa`) قبل أي شي، لأن Cranelift يدعم أكثر من هدف (x86-64, ARM, RISC-V).

---

## الخطوة 3: Signature — بطاقة تعريف الدالة

```rust
use cranelift_codegen::ir::{Signature, AbiParam, types};
use cranelift_codegen::isa::CallConv;

let mut sig = Signature::new(CallConv::SystemV);
sig.params.push(AbiParam::new(types::I32));   // أول parameter
sig.params.push(AbiParam::new(types::I32));   // ثاني parameter
sig.returns.push(AbiParam::new(types::I32));  // نوع الإرجاع
```

هذا يقول: **"دالتي تاخذ i32 و i32، وترجع i32"** — بس، ما فيه منطق داخلي لسا.

---

## الخطوة 4: تسجيل الدالة بالـ Module

```rust
use cranelift_module::{Module, Linkage};

let func_id = module.declare_function("add", Linkage::Export, &sig).unwrap();
```

- `"add"` = اسم الدالة
- `Linkage::Export` = يصير مرئي من برّا (لازم لـ `main` مثلاً عشان اللينكر يلقاها)
- `Linkage::Local` = يستخدم بس داخلياً بنفس الملف

---

## الخطوة 5: بناء جسم الدالة (Function + Builder)

```rust
use cranelift_codegen::ir::{Function, UserFuncName};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};

let mut func = Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), sig);
let mut fb_ctx = FunctionBuilderContext::new();
let mut builder = FunctionBuilder::new(&mut func, &mut fb_ctx);
```

`builder` هو اللي راح تستخدمه لأي تعليمة بعدها.

---

## الخطوة 6: Block — أهم مفهوم بكل الموضوع

**قاعدة ذهبية:** كل block لازم ينتهي بـ `return`/`jump`/`brif`. ما فيه "سقوط تلقائي" لبلوك ثاني.

```rust
let block0 = builder.create_block();                          // أنشئ block
builder.append_block_params_for_function_params(block0);      // خله ياخذ params الدالة
builder.switch_to_block(block0);                                // ابدأ تكتب هنا
builder.seal_block(block0);                                    // قفله (خلصنا نعرف مين يدخله)
```

`seal_block` معناها: **"ما فيه أي جهة ثانية بعدها راح تقفز لهذا البلوك"**. لازم تسويها بعد ما تتأكد من هذا.

---

## الخطوة 7: التعليمات (Instructions)

```rust
let v0 = builder.block_params(block0)[0];  // أول parameter
let v1 = builder.block_params(block0)[1];  // ثاني parameter

let sum = builder.ins().iadd(v0, v1);      // sum = v0 + v1
builder.ins().return_(&[sum]);             // رجّع sum
builder.finalize();                         // خلصنا من هذه الدالة
```

كل تعليمة حسابية (`iadd`, `isub`, `imul`) **ترجع Value جديدة** — هذا هو SSA form (كل قيمة تتحدد مرة وحدة بس).

---

## الخطوة 8: التسجيل النهائي بالـ Module

```rust
use cranelift_codegen::Context;

let mut ctx = Context::for_function(func);
module.define_function(func_id, &mut ctx).unwrap();
module.clear_context(&mut ctx);
```

---

## الخطوة 9: إخراج ملف .o

```rust
let product = module.finish();
std::fs::write("add.o", product.emit().unwrap()).unwrap();
```

```bash
cc add.o -o program   # لازم يكون فيه main بالكود عشان يشتغل!
```

---

## خلاصة الترتيب اللي لازم تحفظه

```
1. Module              (مرة وحدة، للبرنامج كامل)
2. Signature            (لكل دالة)
3. declare_function     (سجّلها بالـ Module)
4. Function + Builder   (جسم الدالة)
5. create_block → switch_to_block → seal_block
6. .ins().شي() لكل تعليمة
7. finalize()
8. Context::for_function + define_function
9. module.finish() + emit()
```

---

## تمرين

جرب تكتب دالة `main` (بدون parameters) ترجع `10 * 2` (يعني exit code = 20)، مستخدم بس الخطوات فوق.
