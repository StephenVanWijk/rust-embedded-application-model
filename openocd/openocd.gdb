target extended-remote localhost:3333
file target/thumbv7em-none-eabihf/debug/rust-embedded-application-model
monitor arm semihosting enable
load
break main
continue