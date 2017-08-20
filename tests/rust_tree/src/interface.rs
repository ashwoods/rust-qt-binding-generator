/* generated by rust_qt_binding_generator */
#![allow(unknown_lints)]
#![allow(mutex_atomic, needless_pass_by_value)]
#![allow(unused_imports)]
use libc::{c_int, c_uint, c_void};
use types::*;
use std::sync::{Arc, Mutex};
use std::ptr::null;

use implementation::*;

pub struct PersonsQObject {}

#[derive (Clone)]
pub struct PersonsEmitter {
    qobject: Arc<Mutex<*const PersonsQObject>>,
    new_data_ready: fn(*const PersonsQObject, row: c_int, parent: usize),
}

unsafe impl Send for PersonsEmitter {}

impl PersonsEmitter {
    fn clear(&self) {
        *self.qobject.lock().unwrap() = null();
    }
    pub fn new_data_ready(&self, row: c_int, parent: usize) {
        let ptr = *self.qobject.lock().unwrap();
        if !ptr.is_null() {
            (self.new_data_ready)(ptr, row, parent);
        }
    }
}

pub struct PersonsUniformTree {
    qobject: *const PersonsQObject,
    begin_reset_model: fn(*const PersonsQObject),
    end_reset_model: fn(*const PersonsQObject),
    begin_insert_rows: fn(*const PersonsQObject,row: c_int, parent: usize, c_int, c_int),
    end_insert_rows: fn(*const PersonsQObject),
    begin_remove_rows: fn(*const PersonsQObject,row: c_int, parent: usize, c_int, c_int),
    end_remove_rows: fn(*const PersonsQObject),
}

impl PersonsUniformTree {
    pub fn begin_reset_model(&self) {
        (self.begin_reset_model)(self.qobject);
    }
    pub fn end_reset_model(&self) {
        (self.end_reset_model)(self.qobject);
    }
    pub fn begin_insert_rows(&self,row: c_int, parent: usize, first: c_int, last: c_int) {
        (self.begin_insert_rows)(self.qobject,row, parent, first, last);
    }
    pub fn end_insert_rows(&self) {
        (self.end_insert_rows)(self.qobject);
    }
    pub fn begin_remove_rows(&self,row: c_int, parent: usize, first: c_int, last: c_int) {
        (self.begin_remove_rows)(self.qobject,row, parent, first, last);
    }
    pub fn end_remove_rows(&self) {
        (self.end_remove_rows)(self.qobject);
    }
}

pub trait PersonsTrait {
    fn create(emit: PersonsEmitter, model: PersonsUniformTree) -> Self;
    fn emit(&self) -> &PersonsEmitter;
    fn row_count(&self, row: c_int, parent: usize) -> c_int;
    fn can_fetch_more(&self, c_int, usize) -> bool { false }
    fn fetch_more(&mut self, c_int, usize) {}
    fn sort(&mut self, c_int, SortOrder) {}
    fn user_name(&self, row: c_int, parent: usize) -> String;
    fn set_user_name(&mut self, row: c_int, parent: usize, String) -> bool;
    fn index(&self, row: c_int, parent: usize) -> usize;
    fn parent(&self, parent: usize) -> QModelIndex;
}

#[no_mangle]
pub extern "C" fn persons_new(qobject: *const PersonsQObject,
        new_data_ready: fn(*const PersonsQObject, row: c_int, parent: usize),
        begin_reset_model: fn(*const PersonsQObject),
        end_reset_model: fn(*const PersonsQObject),
        begin_insert_rows: fn(*const PersonsQObject,row: c_int, parent: usize,
            c_int,
            c_int),
        end_insert_rows: fn(*const PersonsQObject),
        begin_remove_rows: fn(*const PersonsQObject,row: c_int, parent: usize,
            c_int,
            c_int),
        end_remove_rows: fn(*const PersonsQObject))
        -> *mut Persons {
    let emit = PersonsEmitter {
        qobject: Arc::new(Mutex::new(qobject)),
        new_data_ready: new_data_ready,
    };
    let model = PersonsUniformTree {
        qobject: qobject,
        begin_reset_model: begin_reset_model,
        end_reset_model: end_reset_model,
        begin_insert_rows: begin_insert_rows,
        end_insert_rows: end_insert_rows,
        begin_remove_rows: begin_remove_rows,
        end_remove_rows: end_remove_rows,
    };
    let d = Persons::create(emit, model);
    Box::into_raw(Box::new(d))
}

#[no_mangle]
pub unsafe extern "C" fn persons_free(ptr: *mut Persons) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn persons_row_count(ptr: *const Persons, row: c_int, parent: usize) -> c_int {
    (&*ptr).row_count(row, parent)
}
#[no_mangle]
pub unsafe extern "C" fn persons_can_fetch_more(ptr: *const Persons, row: c_int, parent: usize) -> bool {
    (&*ptr).can_fetch_more(row, parent)
}
#[no_mangle]
pub unsafe extern "C" fn persons_fetch_more(ptr: *mut Persons, row: c_int, parent: usize) {
    (&mut *ptr).fetch_more(row, parent)
}
#[no_mangle]
pub unsafe extern "C" fn persons_sort(ptr: *mut Persons, column: c_int, order: SortOrder) {
    (&mut *ptr).sort(column, order)
}

#[no_mangle]
pub unsafe extern "C" fn persons_data_user_name(ptr: *const Persons,
                                    row: c_int, parent: usize,
        d: *mut c_void,
        set: fn(*mut c_void, QString)) {
    let data = (&*ptr).user_name(row, parent);
    set(d, QString::from(&data));
}

#[no_mangle]
pub unsafe extern "C" fn persons_set_data_user_name(ptr: *mut Persons, row: c_int, parent: usize, v: QStringIn) -> bool {
    (&mut *ptr).set_user_name(row, parent, v.convert())
}

#[no_mangle]
pub unsafe extern "C" fn persons_index(ptr: *const Persons, row: c_int, parent: usize) -> usize {
    (&*ptr).index(row, parent)
}
#[no_mangle]
pub unsafe extern "C" fn persons_parent(ptr: *const Persons, parent: usize) -> QModelIndex {
    (&*ptr).parent(parent)
}
