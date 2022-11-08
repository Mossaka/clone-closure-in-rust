
#[derive(Default)]
pub struct Ctx {
    pub count: u32,
}

pub fn call(ctx: &mut Ctx, myfn: impl MyFn) {
    myfn(ctx);
}

pub trait MyFn: FnOnce(&mut Ctx) + Send + 'static {}

impl<T: FnOnce(&mut Ctx) + Send + 'static> MyFn for T {}

fn main() {
    let mut ctx = Ctx::default();
    let c = 5;
    call(&mut ctx, move |state: &mut Ctx| {
        state.count += c;
    });
    println!("count: {}", ctx.count);
}