#[macro_use]
extern crate neon;
extern crate libsyntax2;

use libsyntax2::{
    TextRange,
    File,
    utils::dump_tree,
    SyntaxKind::*,
    algo,
};
use neon::prelude::*;

pub struct Wrapper {
    inner: File,
}

impl Wrapper {
    fn highlight(&self) -> Vec<(TextRange, &'static str)> {
        let mut res = Vec::new();
        let syntax = self.inner.syntax();
        for node in algo::walk::preorder(syntax.as_ref()) {
            if node.kind() == ERROR {
                res.push((node.range(), "error"))
            }
        }
        res
    }
}



declare_types! {
    /// A class for generating greeting strings.
    pub class RustFile for Wrapper {
        init(mut cx) {
            let text = cx.argument::<JsString>(0)?.value();
            Ok(Wrapper {
                inner: File::parse(&text)
            })
        }

        method syntaxTree(mut cx) {
            let this = cx.this();
            let tree = {
                let guard = cx.lock();
                let wrapper = this.borrow(&guard);
                dump_tree(&wrapper.inner.syntax())
            };
            Ok(cx.string(tree.as_str()).upcast())
        }

        method highlight(mut cx) {
            let this = cx.this();
            let highlights = {
                let guard = cx.lock();
                let wrapper = this.borrow(&guard);
                wrapper.highlight()
            };
            let res = cx.empty_array();
            for (i, (range, tag)) in highlights.into_iter().enumerate() {
                let start: u32 = range.start().into();
                let end: u32 = range.end().into();
                let start = cx.number(start);
                let end = cx.number(end);
                let tag = cx.string(tag);
                let hl = cx.empty_array();
                hl.set(&mut cx, 0, start)?;
                hl.set(&mut cx, 1, end)?;
                hl.set(&mut cx, 2, tag)?;
                res.set(&mut cx, i as u32, hl)?;
            }

            Ok(res.upcast())
        }
    }

}

register_module!(mut cx, {
    cx.export_class::<RustFile>("RustFile")
});
