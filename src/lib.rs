#[macro_export]
macro_rules! cfor {
    ($var:stmt; $test:expr; $increment:expr, $body:block) => {
        {
            $var
            while ($test) {
                $body
                $increment;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use core::arch::asm;

    #[test]
    fn it_works() {
        let mut control = 0;
        let mut range_for = 0;
        let mut for_each = 0;

        (0..10).for_each(|_| {
            for_each += 1;
        });

        for _ in 0..10 {
            range_for += 1;
        }

        cfor!(let mut i=0; i < 10; i += 1, {
            control += 1;
        });

        assert!(control == range_for && control == for_each);
    }

    #[test]
    fn explore() {
        let mut i: u64 = 0;
        let mut result = 0;

        // SAFETY: It's just a for loop
        unsafe {
            asm!(
                "MOV {tmp}, {i}",
                "MOV {accu:e}, 0",
                "2:",
                "INC {tmp}",
                "ADD {accu:e}, 2",
                "CMP {tmp}, 10",
                "JL 2b",
                "MOV {i}, {tmp}",
                i = inout(reg) i,
                tmp = out(reg) _,
                accu = out(reg) result,
            );
        }

        assert_eq!(i, 10);
        assert_eq!(result, 20);
    }
}
