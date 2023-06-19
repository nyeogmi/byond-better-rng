@REM NOTE: Don't forget to rustup target add i686-pc-windows-msvc
cargo rustc --target=i686-pc-windows-msvc --release -- -C target-cpu=native
copy target\i686-pc-windows-msvc\release\better_rng.dll .
copy better_rng.dll ..\dreammaker_testing