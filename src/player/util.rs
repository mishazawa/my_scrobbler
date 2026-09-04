use objc2::{rc::Retained, runtime::AnyObject};
use objc2_foundation::{NSDictionary, NSNumber, NSString};

pub trait FromUserInfoValue: Sized {
    fn extract(obj: &Retained<AnyObject>) -> Option<Self>;
}

impl FromUserInfoValue for String {
    fn extract(obj: &Retained<AnyObject>) -> Option<Self> {
        obj.downcast_ref::<NSString>().map(|s| s.to_string())
    }
}

impl FromUserInfoValue for f64 {
    fn extract(obj: &Retained<AnyObject>) -> Option<Self> {
        obj.downcast_ref::<NSNumber>().map(|n| n.as_f64())
    }
}

pub fn get_value_from_user_info<T: FromUserInfoValue>(
    ui: &Retained<NSDictionary>,
    key: &str,
    default: T,
) -> T {
    let k = NSString::from_str(key);
    ui.objectForKey(&k)
        .and_then(|v| T::extract(&v))
        .unwrap_or(default)
}
