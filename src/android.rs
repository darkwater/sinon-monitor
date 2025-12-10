use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

#[cfg(target_os = "android")]
mod inner {
    pub fn battery_level() -> i32 {
        use jni::objects::JObject;

        let android = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(android.vm().cast()) }.unwrap();
        let mut env = vm.attach_current_thread().unwrap();
        let class_ctx = env.find_class("android/content/Context").unwrap();

        let battery_service = env
            .get_static_field(class_ctx, "BATTERY_SERVICE", "Ljava/lang/String;")
            .unwrap();

        let context = unsafe { JObject::from_raw(android.context().cast()) };

        let battery_manager = env
            .call_method(
                context,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[battery_service.borrow()],
            )
            .unwrap()
            .l()
            .unwrap();

        let class_battery = env.find_class("android/os/BatteryManager").unwrap();
        let capacity = env
            .get_static_field(class_battery, "BATTERY_PROPERTY_CAPACITY", "I")
            .unwrap();

        env.call_method(
            battery_manager,
            "getIntProperty",
            "(I)I",
            &[capacity.borrow()],
        )
        .unwrap()
        .i()
        .unwrap()
    }
}

#[cfg(not(target_os = "android"))]
mod inner {
    pub fn battery_level() -> i32 {
        90
    }
}

pub fn battery_level() -> i32 {
    static CACHE: Mutex<Option<(Instant, i32)>> = Mutex::new(None);
    const CACHE_DURATION: Duration = Duration::from_secs(30);

    let mut cache = CACHE.lock().unwrap();
    let now = Instant::now();

    if let Some((timestamp, level)) = *cache
        && now.duration_since(timestamp) < CACHE_DURATION
    {
        return level;
    }

    let level = inner::battery_level();
    *cache = Some((now, level));
    level
}
