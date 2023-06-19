#![feature(once_cell)]
#![feature(pointer_byte_offsets)]
use auxtools::{init, sigscan, BYONDCORE, signature};
use detour::RawDetour;
use rand::Rng;
use std::{cell::{UnsafeCell}, mem::MaybeUninit};
use rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;

static RNG: WrappedRng = WrappedRng { rng: UnsafeCell::new(MaybeUninit::uninit()) };

struct WrappedRng { rng: UnsafeCell<MaybeUninit<ChaCha8Rng>> }
unsafe impl Sync for WrappedRng {}

#[init(full)]
fn _init_rng() -> Result<(), std::string::String> {
    // initialize rng
    unsafe {
        *RNG.rng.get() = MaybeUninit::new(ChaCha8Rng::from_entropy());
    }

    // this is the same boilerplate as instruction_hooking used
    // in auxtools
    let byondcore = sigscan::Scanner::for_module(BYONDCORE).unwrap();

    let (p0, p1, p2) = if cfg!(windows) {
        // NOTE: These differ mostly in minor details of their prologues.
        // Find a better way to identify them?
        (
            byondcore.find(signature!(
                // "ask mersenne for an f32 [0.0,1.0)"
                // push ebp
                // mov ebp, esp
                // push ecx
                // push esi
                // push edi
                // mov edi, ecx
                // cmp dword ptr ds:[edi+????], 0
                // jne <after call>
                // call <inner body of rng>
                // mov ecx, dword ptr ds:[edi+????] 
                // xor edx, edx 
                // mov esi, dword ptr ds:[edi+ecx*4]
                // mov eax, esi
                // shr eax, B
                // xor esi, eax
                // mov eax, esi
                // and eax, FF3A58AD
                "55 8B EC 51 56 57 8B F9 83 BF ?? ?? ?? ?? 00 75 05 E8 ?? ?? ?? ?? 8B 8F ?? ?? ?? ?? 33 D2 8B 34 8F 8B C6 C1 E8 0B 33 F0 8B C6 25 AD 58 3A FF"
            )).ok_or_else(|| "Couldn't find the Mersenne Twister (rand f32)")?,

            unsafe { byondcore.find(signature!(
                // "ask mersenne for a u32"
                // int3  (included because this is buried in the other two)
                // push esi
                // push edi
                // mov edi, ecx
                // cmp dword ptr ds:[edi+????], 0
                // jne <after call>
                // call <inner body of rng>
                // mov ecx,dword ptr ds:[edi+????] 
                // xor edx, edx
                // mov esi, dword ptr ds:[edi+ecx*4]
                // mov eax, esi
                // shr eax, B
                // xor esi, eax
                // mov eax, esi
                // and eax, FF3A58AD
                "CC 56 57 8B F9 83 BF ?? ?? ?? ?? 00 75 05 E8 ?? ?? ?? ?? 8B 8F ?? ?? ?? ?? 33 D2 8B 34 8F 8B C6 C1 E8 0B 33 F0 8B C6 25 AD 58 3A FF")).ok_or_else(|| "Couldn't find the Mersenne Twister (rand u32)")?.byte_add(1) },
            byondcore.find(signature!(
                // "ask mersenne for an f64 [0.0,1.0)"
                // push ebp
                // mov ebp, esp
                // sub esp, 8
                // push esi
                // push edi
                // mov edi, ecx
                // cmp dword ptr ds:[edi+????], 0
                // jne <after call>
                // call <inner body of rng>
                // mov ecx, dword ptr ds:[edi+????] 
                // xor edx, edx
                // mov esi, dword ptr ds:[edi+ecx*4]
                // mov eax, esi
                "55 8B EC 83 EC 08 56 57 8B F9 83 BF ?? ?? ?? ?? 00 75 05 E8 ?? ?? ?? ?? 8B 8F ?? ?? ?? ?? 33 D2 8B 34 8F 8B C6"
            )).ok_or_else(|| "Couldn't find the Mersenne Twister (rand f64)")?,
        )
    } else if cfg!(unix) {
        return Err("TODO: Support Linux".to_string())
    } else {
        return Err("What platform is this?".to_string())
    };

    unsafe {
        let hook0 = RawDetour::new(
            p0 as *const (),
            rng_hook_f32 as *const (),
        ).map_err(|_| "Couldn't detour the Mersenne Twister (f32)")?;

        hook0.enable().map_err(|_| "Couldn't enable the Mersenne Twister detour (f32)")?;
        std::mem::forget(hook0);

        let hook1 = RawDetour::new(
            p1 as *const (),
            rng_hook_u32 as *const (),
        ).map_err(|_| "Couldn't detour the Mersenne Twister (u32)")?;

        hook1.enable().map_err(|_| "Couldn't enable the Mersenne Twister detour (u32)")?;
        std::mem::forget(hook1);

        let hook2 = RawDetour::new(
            p2 as *const (),
            rng_hook_f64 as *const (),
        ).map_err(|_| "Couldn't detour the Mersenne Twister (f64)")?;

        hook2.enable().map_err(|_| "Couldn't enable the Mersenne Twister detour (f64)")?;
        std::mem::forget(hook2);
    }

    Ok(())
}

fn rng_hook_f32() -> f32 {
    return unsafe { (*RNG.rng.get()).assume_init_mut().gen::<f32>() }
}

fn rng_hook_u32() -> u32 {
    return unsafe { (*RNG.rng.get()).assume_init_mut().gen::<u32>() }
}

fn rng_hook_f64() -> f64 {
    return unsafe { (*RNG.rng.get()).assume_init_mut().gen::<f64>() }
}