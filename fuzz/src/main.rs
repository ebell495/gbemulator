use lib_gbemulation::apu::apu::Apu;
use lib_gbemulation::apu::*;
use lib_gbemulation::cartridge;
use lib_gbemulation::cpu::cpu::Cpu;
use lib_gbemulation::gpu::*;
use lib_gbemulation::gpu::gpu::Gpu;
use lib_gbemulation::io::joypad::Joypad;
use lib_gbemulation::memory::mmu::Mmu;
use std::sync::Arc;
use std::io::Read;
use std::io::BufReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file = std::fs::File::open(&args[1]).expect("Bad Filename");
        let mut data_vec = Vec::new();
        let mut reader = BufReader::new(file);
    
        let read_result = reader.read_to_end(&mut data_vec);
        match read_result {
            Ok(_)=> {
                let slice_fuzz_data = & data_vec[..];
                fuzz_target(slice_fuzz_data);
            },
            Err(_)=>panic!("Bad file read")
        }
    }
}

struct DummyScreen {}
impl Screen for DummyScreen {
    fn draw(&self, _screen_buffer: &[u8; BUFFER_SIZE]) {}
    fn get_palette(&self) -> [[u8; 3]; 4] {
        return [
            [8, 24, 32],
            [52, 104, 86],
            [136, 192, 112],
            [224, 248, 208]
        ]
    }
}

struct DummyAudio {}
impl AudioOutput for DummyAudio {
    fn output(&mut self, _sample: (i16, i16)) {}
    fn get_sample_rate(&self) -> u32 {
        return 44100;
    }
}

fn fuzz_target(data: &[u8]) {
    let mut cart_data = vec![0; 65536 as usize];
    cart_data[0..data.len()].copy_from_slice(data);
    println!("{:?}", cart_data);
    let cartridge_res = cartridge::new_cartridge(cart_data, None);
    match cartridge_res {
        Ok(mut cartridge)=>{
            let mut dummy_audio = DummyAudio {};
            let dummy_screen = DummyScreen {};
            
        
            let mut apu = Apu::new(&mut dummy_audio);
            let mut gpu = Gpu::new(Arc::new(dummy_screen));
            let mut mmu = Mmu::new(&mut *cartridge, &mut gpu, &mut apu);
            let mut cpu = Cpu::new();
            let joypad = Joypad::new();
            let mut emulation = lib_gbemulation::emulation::Emulation::new();
            for _ in 0..100 {
                emulation.cycle(&mut cpu, &mut mmu, &joypad);
            }
        },
        _=>()
    }
}