use std::{env, io::{self, Write}, net::{IpAddr, TcpStream}, process, str::FromStr, sync::mpsc::{channel, Sender}, thread};

const MAX_PORT: u16= 65535;

struct Arguements{
    ip_addr : IpAddr,
    num_threads : u16,
}

impl Arguements{
    fn new(args : &[String]) -> Result<Arguements,&'static str>{
        if args.len() < 2{
            return Err("Too few arguements");
        }
        else if args.len() > 4{
            return Err("Too many arguments");
        }
        let f:String = args[1].clone();
        
        if let Ok(ip_addr) = IpAddr::from_str(&f){
            return Ok(Arguements{ip_addr : ip_addr,num_threads : 4});
        } 
        else{
            let flag:String = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2{
                println!("-h or --help flag to display this message\n-j flag to specify the number of threads to use");
                return Err("help");
            }
            else if flag.contains("-h") || flag.contains("--help"){
                return Err("too many Arguements");
            }
            else if flag.contains("-j") && args.len() == 4{
                let thread_num : String = args[2].clone();
                let num_threads : u16 = thread_num.parse::<u16>().unwrap();
                let ip_addr_str = args[3].clone();
                let ip_addr = IpAddr::from_str(&ip_addr_str).unwrap();
                return Ok(Arguements{ip_addr : ip_addr,num_threads : num_threads});
            }
            else{
                return Err("invalid syntax");
            }
        }
    }
}


fn scan(tx : Sender<u16>,start_port : u16,addr: IpAddr,num_threads : u16){
    let mut port = start_port;

    loop {
        match TcpStream::connect((addr,port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORT - port) <= num_threads{
            break;
        }
        port += num_threads;
    }
}
fn main(){
    /*
        Usage:
        cargo run -- <ip-address>
        cargo run -- -h
        cargo run -- --help
        cargo run -- -j <num> <ip-address>
     */

    let args:Vec<String> = env::args().collect();
    let program :String = args[0].clone();
    let arguements = Arguements::new(&args).unwrap_or_else(|err| {
        if err.contains("help"){
            process::exit(0);
        }
        else{
            eprintln!("{} : error in parsing options : {}",program,err);
            process::exit(1);
        }
    });

    let (tx,rx) = channel();

    let num_threads = arguements.num_threads;
    let addr = arguements.ip_addr;

    for i in 1..=num_threads{
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx,i,addr,num_threads);
        });
    }

    drop(tx);
    let mut outs:Vec<u16> = vec![];

    for recv in rx{
        outs.push(recv);
    }
    println!();
    for port in outs{
        println!("Port {} is open ",port);
    }

}