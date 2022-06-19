use keyvalues_parser::{Obj, Value, Vdf};

use std::{borrow::Cow, collections::BTreeMap};

#[test]
fn simple_vdfs_iteration() {
    let inner = BTreeMap::new();
    let obj = Obj(inner);
    let mut vdfs_iter = obj.into_vdfs();

    assert_eq!(vdfs_iter.next(), None);
}

#[test]
fn complex_vdfs_iteration() {
    let key1 = Cow::from("key1");
    let key4 = Cow::from("key4");
    let val1 = Value::Str(Cow::from("val1"));
    let val2 = Value::Str(Cow::from("val2"));
    let empty_obj = Value::Obj(Obj::new());

    let pairs = vec![
        (key1.clone(), vec![val1.clone(), val2.clone()]),
        (Cow::from("key2"), Vec::new()),
        (Cow::from("key3"), Vec::new()),
        (key4.clone(), vec![empty_obj.clone()]),
    ];

    let obj: Obj = pairs.into_iter().collect();
    let vdfs: Vec<_> = obj.into_vdfs().collect();

    assert_eq!(
        vdfs,
        vec![
            Vdf::new(key1.clone(), val1),
            Vdf::new(key1, val2),
            Vdf::new(key4, empty_obj),
        ]
    );
}
