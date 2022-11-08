#[derive(Default)]
pub struct Ctx {
    pub count: u32,
}

pub fn add_to_state(builder: &mut StateBuilder, get_cx: impl MyFn) {
    builder.get_cx_fns.push(Box::new(get_cx));
}

#[derive(Clone, Default)]
pub struct StateBuilder {
    pub get_cx_fns: Vec<Box<dyn MyFn>>,
}
pub trait MyFn: Fn(&mut Ctx) + MyFnClone + Send + 'static {
}

impl<T: Fn(&mut Ctx) + Send + Clone + 'static> MyFn for T {}

pub trait MyFnClone {
    fn clone_box(&self) -> Box<dyn MyFn>;
}

impl<T> MyFnClone for T
where
    T: 'static + Clone + MyFn,
{
    fn clone_box(&self) -> Box<dyn MyFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MyFn> {
    fn clone(&self) -> Box<dyn MyFn> {
        self.clone_box()
    }
}

impl StateBuilder {
    pub fn build(self) -> Ctx {
        let mut ctx = Ctx::default();
        for fns in self.get_cx_fns {
            let _ = fns(&mut ctx);
        }
        ctx
    }
}

fn main() {
    let mut state_builder = StateBuilder::default();
    let c = 5;
    let d = 10;
    add_to_state(&mut state_builder, move |state: &mut Ctx| {
        state.count += c;
    });
    add_to_state(&mut state_builder, move |state: &mut Ctx| {
        state.count += d;
    });

    let ctx = state_builder.build();
    println!("count: {}", ctx.count);
}