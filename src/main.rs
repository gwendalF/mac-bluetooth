use apple_sys::IOBluetooth::{IOBluetoothDeviceInquiry, PIOBluetoothDeviceInquiry};
use objc::{
    class,
    runtime::{Object, BOOL},
    sel, ClassDecl,
};

extern "C" fn inquiry_complete(_this: &Object, _sel: Sel, error: IOReturn, aborted: BOOL) {
    println!("Completed");
}

fn main() {
    println!("Hello, world!");
    let superclass = class!(NSObject);
    let mut decl = ClassDecl::new("MyInquiryDelegate", superclass).unwrap();
    let inquiry = IOBluetoothDeviceInquiry(IOBluetoothDeviceInquiry::alloc().initWithDelegate_());
}
