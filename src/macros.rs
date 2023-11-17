macro_rules! forward {
    (
        $(
            $method:ident($($arg:ident : $arg_ty:ty),*) => $function:path;
        )*
    ) => {
        $(
            #[inline]
            fn $method(&mut self $(, $arg : $arg_ty)*) -> Result<(), Self::Error>
            {
                self.emit_slice(&$function($($arg),*).to_le_bytes())
            }
        )*
    };
}

pub(crate) use forward;
