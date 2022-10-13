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
}
