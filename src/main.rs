extern crate ncurses;
use ncurses::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	initscr();
	raw();
	keypad(stdscr(), true);
	noecho();
	halfdelay(10);

	let cores = setup_program();

	mvprintw(0, 0, "[[cpufreq_monitor]]");
	let avail_gov = get_governor_info();
	mvprintw(1, 0, "available governors:");
	mvprintw(1, 25, &avail_gov);
	let curr_gov = get_current_governor();
	mvprintw(2, 0, "current governor:");
	mvprintw(2, 25, &curr_gov); 

	for i in 0..cores {
		let mut cpu_name = String::from("cpu");
		let index = i.to_string();
		cpu_name = cpu_name + &index;
		mvprintw(i+4, 0, &cpu_name);
	}

	loop {
		let ch = getch();
		if ch == KEY_F1 {
			break;
		}	
		for i in 0..cores {
			let value = get_frequency(i);
			mvprintw(i+4, 10, &value);	
		}		
	}
   endwin(); 
}

fn setup_program() -> i32 {
	let mut count = 0;
	loop {
		let mut top = String::from("/sys/devices/system/cpu/");
		let mut sub = String::from("cpu");
		let num = count.to_string();
		sub += &num;
		top += &sub;
		let does_exit = Path::new(&top).exists();
		if !does_exit {
			break;
		}
		count += 1;
	}
	count
} 

fn get_frequency(index: i32) -> String {
	let mut filename = String::from("/sys/devices/system/cpu/cpu");
	let num = index.to_string();
	let frequency: String = String::from("/cpufreq/cpuinfo_cur_freq");
	filename = filename + &num + &frequency;
	let content = read_file(filename);
	content
}

fn get_governor_info() -> String {
	let path = String::from("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors");
	read_file(path)
}

fn get_current_governor() -> String {
	let path = String::from("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor");
	read_file(path)
}

fn read_file(file_path: String) -> String {
	let mut read_file = File::open(file_path)
		.expect("error reading");
	let mut contents = String::new();
	read_file.read_to_string(&mut contents)
		.expect("error writing to string");
	contents
}
