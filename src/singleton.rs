#[macro_export]
macro_rules! singleton {
    // 基本形式：定义单例结构并提供访问方法
    ($struct_name:ident : $instance_name:ident = $init:expr) => {
        // 定义静态实例
        static mut $instance_name: $struct_name = $init;

        // 为结构实现单例方法
        impl $struct_name {
            #[inline]
            pub const fn single() -> &'static Self {
                #[allow(static_mut_refs)]
                unsafe {
                    &$instance_name
                }
            }

            #[inline]
            pub const fn single_mut() -> &'static mut Self {
                #[allow(static_mut_refs)]
                unsafe {
                    &mut $instance_name
                }
            }
        }
    };

    // 简化形式：使用默认的实例名称
    ($struct_name:ident = $init:expr) => {
        singleton!($struct_name : INSTANCE = $init);
    };
}
