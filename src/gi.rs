use std::ffi::CString;

use libc::c_char;

use glib::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::ToValue;

use crate::version::get_fin_version;

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("version", |version| {
    glib::ParamSpec::string(
        version,
        "Version",
        "Version",
        None,
        glib::ParamFlags::READABLE,
    )
})];

pub struct RustClient {
    version: CString,
}

type FinClientInstance = subclass::simple::InstanceStruct<RustClient>;
type FinClientClass = subclass::simple::ClassStruct<RustClient>;

impl ObjectSubclass for RustClient {
    const NAME: &'static str = "FinClient";

    type ParentType = glib::Object;

    type Instance = FinClientInstance;
    type Class = FinClientClass;

    glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(&PROPERTIES);
    }

    fn new() -> Self {
        Self {
            version:CString::new(&get_fin_version() as &str).unwrap(),
        }
    }
}

impl ObjectImpl for RustClient {
    glib_object_impl!();

    fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];

        match *prop {
            subclass::Property("version", ..) => {
                Ok(self.version.clone().into_string().unwrap().to_value())
            },
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }
}

impl RustClient {
    fn get_version(&self) -> *const libc::c_char {
        self.version.as_ptr()
    }
}

glib_wrapper! {
    pub struct ClientWrapper(Object<FinClient, FinClientClass, ClientClass>);

    match fn {
        get_type => || RustClient::get_type().to_glib(),
    }
}

#[repr(C)]
pub struct FinClient {
    parent: gobject_sys::GObject,
}

unsafe impl InstanceStruct for FinClient {
    type Type = RustClient;
}

fn into_rust_client<'a>(client: *const FinClient) -> &'a RustClient {
    unsafe { &*client }.get_impl()
}

#[no_mangle]
unsafe extern "C" fn fin_client_new() -> *mut FinClient {
    let obj = glib::Object::new(
        RustClient::get_type(),
        &[],
    )
    .unwrap()
    .downcast::<ClientWrapper>()
    .unwrap();
    obj.to_glib_full()
}

#[no_mangle]
unsafe extern "C" fn fin_client_get_type() -> glib_sys::GType {
    RustClient::get_type().to_glib()
}

#[no_mangle]
unsafe extern "C" fn fin_client_get_version(this: *const FinClient) -> *const c_char {
    into_rust_client(this).get_version()
}
