// use std::{sync::Arc, rc::Rc};

// use std::any::Any;

// use as_any::{Downcast, AsAny};

// pub trait Resource: AsAny {}

// #[derive(Clone)]
// pub struct Kv {
//     pub key: String,
//     pub value: String,
// }

// impl Kv {
//     pub fn do_something(&mut self) {
//         self.key = "new_key".to_string();
//         println!("do something");
//     }
// }

// impl Resource for Kv {}

// #[derive(Clone)]
// pub struct A {
//     pub rsc: Vec<Arc<dyn Resource + Send + Sync>>,
// }


// fn main() {
//     let mut a = A { rsc: vec![] };
//     a.rsc.push(Arc::new(
//         Kv {
//             key: "key".to_string(),
//             value: "value".to_string(),
//         }
//     ));

//     let b = a.clone();

    

//     for mut resource in b.rsc {
//         let mut kv = Arc::get_mut(&mut resource).unwrap();

//         // downcast kv to Kv
//         let kv = kv.downcast_mut::<Kv>().unwrap();
//         kv.do_something();
//     }

//     println!("Hello, world!");
// }

use std::any::Any;

// use wasmtime::{Engine, Linker, Store};
// use wasmtime_wasi::WasiCtxBuilder;

fn main() {
    let mut state_builder = StateBuilder::default();

    let c = 5;
    add_to_builder(&mut state_builder, |ctx: &mut Ctx| {
        ctx.count += c;
    });


    // add_to_builder(&mut state_builder, |state: &mut Ctx| {
    //     state.count += 1;
    // });


    // add_to_builder(&mut state_builder, |state: &mut Ctx| {
    //     state.count += 1;
    // });

    let ctx = state_builder.build();
    println!("count: {}", ctx.count);
}

trait MyFn<T>: Send + 'static + FnClone 
where
    T: Fn(&mut Ctx)
{}

pub trait FnClone {
    fn clone_box(&self) -> Box<dyn MyFn>;
}

impl<T> FnClone for T
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

#[derive(Clone, Default)]
pub struct Ctx {
    count: u32,
}

#[derive(Clone, Default)]
pub struct StateBuilder {
    get_cx_fns: Vec<Box<dyn MyFn>>,
}

impl StateBuilder {
    pub fn build(self) -> Ctx {
        let mut ctx = Ctx::default();
        for fns in self.get_cx_fns {
            let res = fns(&mut ctx);
            println!("res: {:?}", res);
        }
        ctx
    }
}

fn add_to_builder(builder: &mut StateBuilder, get_cx: impl MyFn) -> () {
    builder.get_cx_fns.push(
        Box::new(get_cx)
    );
}