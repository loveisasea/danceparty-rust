extern crate rand;
//extern crate time;

use std::io; 
use std::sync::{Arc, Mutex}; 
use std::thread;
use std::sync::mpsc::{Sender,Receiver,channel}; 
use std::fmt;
use rand::distributions::{IndependentSample, Range};
   
struct DanceType {
	id   : i32,
	name : String
}
 

impl fmt::Display for DanceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        try!(write!(f, "<dance:{},{}>", self.id,self.name)); 
        Ok(())
    }
} 
 
struct Leader{
	id :i32,
	dance_confirmed: Vec<i32>,
	senders: Vec<Arc<Mutex<Sender<Invitation>>>>,
	receiver: Receiver<InviResult>
}
 
impl fmt::Display for Leader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        try!(write!(f, "<leader:{:?}>", self.id)); 
        Ok(())
    }
} 

impl Leader {
	fn new(id :i32,dance_cnt: i32) -> Leader{
		let (_,rx) = channel();
		let mut leader = Leader{id:id, 
			dance_confirmed: Vec::<i32>::new() , 
			senders: Vec::<Arc<Mutex<Sender<Invitation>>>>::new(),
			receiver: rx
			};
		for _ in 0..dance_cnt {
			leader.dance_confirmed.push(-1);
		} 
		leader
	} 
	
	fn run(&mut self ){ 
		println!("{} is ready", self);
		let mut rng = rand::thread_rng();  
		//随机选取dance_type
		let mut dances = (0i32..self.dance_confirmed.len() as i32).map(move |x| x).collect::<Vec<_>>();
		let between = Range::new(0 as usize, dances.len()); 
		for i in (0..dances.len()){
			let a = between.ind_sample(&mut rng);
			let tmp = dances[i];
			dances[i] = dances[a as usize];
			dances[a as usize] = tmp;
		}  
		for dance_type_id in &dances {
			//随机选取follower
			let mut senders = (0..self.senders.len() as i32).collect::<Vec<i32>>();
			let between2 = Range::new(0 as usize, senders.len()); 
			for i in (0..senders.len()){
				let a = between2.ind_sample(&mut rng);
				let tmp = senders[i];
				senders[i] = senders[a as usize];
				senders[a as usize] = tmp;
			} 
			 for follower_id in &senders{
			 	//发送邀请 
			 	let inv = Invitation{leader_id: self.id,
			 		follower_id: *follower_id, 
			 		dance_type_id: *dance_type_id 
		 		} ;  
		 		println!("{}发送邀请{}...",self,inv); 
		 		match self.senders[*follower_id as usize].lock().unwrap().send(inv){
		 			Err(e) => { println!("{}发送邀请失败，原因：{}",self,e);continue;}
		 			_ => {}
		 		} 
		 		
		 		//接受并处理结果
		 		let res = match self.receiver.recv(){
		 			Err(e) => { println!("{}接收回应失败，原因：{}",self,e);continue;}
		 			Ok(res) => res
		 		};
		 		match res {
		 			InviResult::Init => println!("{}等待超时{}", self,inv), 
		 			InviResult::Accept => {
		 				self.dance_confirmed[inv.dance_type_id as usize] = inv.follower_id;
						println!("{}收到了接受的回应{}====", self, inv);
						break;
		 			}
		 			InviResult::Reject => println!("{}收到了拒绝邀请的回应{}====", self,inv)
		 		} 
			 }
		}
	}
}

 
struct Follower{
	id :i32, 
	dance_confirmed: Vec<i32>,
	leader_dance: Vec<i32>,
	senders: Vec<Arc<Mutex<Sender<InviResult>>>>,
	receiver: Receiver<Invitation>,
	finish: bool
}

impl fmt::Display for Follower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        try!(write!(f, "<follower:{}>", self.id)); 
        Ok(())
    }
} 

impl Follower{ 
	fn new(id :i32,dance_cnt :i32,leader_cnt :i32 ) -> Follower{
		let (_,rx) = channel();
		let mut follower = Follower{id:id, 
			dance_confirmed: Vec::<i32>::new(), 
			leader_dance: Vec::<i32>::new(),
			senders: Vec::<Arc<Mutex<Sender<InviResult>>>>::new(),
			receiver: rx,
			finish: false
			};
		for _ in 0..dance_cnt {
			follower.dance_confirmed.push(-1);
		} 
		for _ in 0..leader_cnt {
			follower.leader_dance.push(0);
		}
		follower 
	}  
	
	fn reply(&self, inv :&Invitation, res :&mut InviResult){ 
		if self.dance_confirmed[inv.dance_type_id as usize] >=0 {
			*res = InviResult::Reject;
			println!("{}拒绝邀请，因为已和{}参与过舞蹈{}",self,self.dance_confirmed[inv.dance_type_id as usize],inv.dance_type_id);
		} 
		else if self.leader_dance[inv.leader_id as usize] >= 2 {
			*res = InviResult::Reject;
			println!("{}拒绝邀请，因为已接受过{}的{}次邀请",self,inv.leader_id,2);
		}
		else{
			*res = InviResult::Accept;
			println!("{}接受邀请{}",self, *inv);
		}  
	}
	
	fn run(&mut self){ 
		self.finish = false;
		println!("{} is ready",self);
		while !self.finish {   
			let inv =  match self.receiver.recv() { 
				Err(e) =>{println!("{}接收时发生错误，原因:{}",self,e);continue;}
				Ok(inv) => {
					match inv.leader_id{
						-1 => {self.finish=true;continue;}
						_ => inv
					}
				}
			};
			println!("{}收到邀请{}",self,inv);
			let mut res = InviResult::Init;
			self.reply(& inv,&mut res);
			if res == InviResult::Accept {
				self.dance_confirmed[inv.dance_type_id as usize] = inv.leader_id;
				self.leader_dance[inv.leader_id as usize] +=1;
			}
			match self.senders[inv.leader_id as usize].lock().unwrap().send(res){
	 			Err(e) => { println!("{}发送回应失败，原因：{}",self,e);continue;}
	 			_ => {}
	 		} 
		} 		 
	}
}

//impl <'a > Follower<'a > {
//	fn new(id : i32, danceParty: & DanceParty) -> &'a Follower<'a >{
//		&'a Follower{id:id,danceParty:danceParty,dance_confirmed:dance_confirmed, }
//	}
//}

#[derive(Debug,PartialEq)]
enum InviResult{
	Init,
	Reject,
	Accept
}

#[derive(Debug,Clone,Copy)]
struct Invitation{
	leader_id : i32,
	follower_id : i32,
	dance_type_id: i32 
}

impl fmt::Display for Invitation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        try!(write!(f, "<invitation:{:?},{:?},{:?}>", self.leader_id, self.follower_id, self.dance_type_id)); 
        Ok(())
    }
}  


   
fn main() {  
	let  dance_types   = [
		DanceType{id:0,name:"Waltz".to_string()},
		DanceType{id:1,name:"Tango".to_string()},
		DanceType{id:2,name:"Foxtrot".to_string()},
		DanceType{id:3,name:"Quickstep".to_string()},
		DanceType{id:4,name:"Rumba".to_string()},
		DanceType{id:5,name:"Samba".to_string()},
		DanceType{id:6,name:"ChaCha".to_string()},
		DanceType{id:7,name:"Jive".to_string()} 
	];   
		let leader_cnt : i32 = {
			let mut leader_cnt_input = String::new();
			println!("Please input the leader number(type 'q' to quit):");
			match io::stdin().read_line(&mut leader_cnt_input) {
				Err(e) => { println!("输入有误{}",e);return;} 
				Ok(_)=> {
					match leader_cnt_input.trim(){
						"q" => return,
						str => match str.parse::<i32>(){
							Err(e) => { println!("{}",e);return;}
							Ok(i)=> i
							}
						}
					}
			} 
		};
		
		let follower_cnt : i32 = {
			let mut follower_cnt_input = String::new();
			println!("Please input the follower number(type 'q' to quit):");
			match io::stdin().read_line(&mut follower_cnt_input) {
				Err(e) => { println!("输入有误{}",e);return;} 
				Ok(_)=> {
					match follower_cnt_input.trim(){
						"q" => return,
						str => match str.parse::<i32>(){
							Err(e) => { println!("{}",e);return;}
							Ok(i)=> i
							}
						}
					}
			} 
		};
		 
		 
		 
		let followers = (0..follower_cnt).map(|i|
			Arc::new(Mutex::new(Follower::new(i,dance_types.len() as i32,leader_cnt)))
		).collect::<Vec<_>>();
		
		
		let leaders = (0..leader_cnt).map(|i|
			Arc::new(Mutex::new(Leader::new(i,dance_types.len() as i32)))
		).collect::<Vec<_>>();
		
		 
		
		for follower in &followers {
			let (tx,rx) = channel();
			follower.lock().unwrap().receiver = rx;
			let sender = Arc::new(Mutex::new(tx));
			for leader in &leaders {
				leader.lock().unwrap().senders.push(sender.clone());
			}
		}
		 
		
		for leader in &leaders {
			let (tx,rx) = channel();
			leader.lock().unwrap().receiver = rx;
			let sender = Arc::new(Mutex::new(tx));
			for follower in &followers {
				follower.lock().unwrap().senders.push(sender.clone());
			}
		}
		
		
		let follower_handlers = followers.iter().map(|follower| {
			let follower = follower.clone(); 
			thread::spawn(move || {	
					let mut follower = follower.lock().unwrap();
					follower.run();
					})
			}
			).collect::<Vec<_>>();
		
		let leader_handlers = leaders.iter().map(|leader| {
				let leader = leader.clone();
				 thread::spawn(move || { 
					let mut leader = leader.lock().unwrap();    
					leader.run();
					})
				}
			    ).collect::<Vec<_>>(); 
		 
		 
		for handler in leader_handlers {
			match handler.join(){
				Err(e) => println!("线程结束时出错，原因:{:?}",e),
				Ok(_) => {}
			}
		} 
		println!("leader已全部结束！");
		
		if followers.len() > 0 && leaders.len() > 0 {
			let leader   = leaders[0].clone();
			let leader = leader.lock().unwrap();
			for sender in &leader.senders {
				match sender.lock().unwrap().send(Invitation{leader_id :-1,follower_id :-1, dance_type_id :-1}){
					Err(e) => {println!("发送空邀请时错误，原因:{}",e);}
					_ => {}
				}
			}
		}
		
		for handler in follower_handlers {
			match handler.join(){
				Err(e) => println!("线程结束时出错，原因:{:?}",e),
				Ok(_) => {}
			}
		} 
		println!("follower已全部结束！");
		
		for i in (0..leader_cnt) {
			let leader = leaders[i as usize].lock().unwrap();
			println!("Leader :{:?}" ,i);
			for j in (0..leader.dance_confirmed.len()) {
				let follower_id = leader.dance_confirmed[j];
				if  follower_id < 0  {
					println!("{:35} with --", dance_types[j]);
				}
				else {
					println!("{:35} with {:?}", dance_types[j], follower_id);
				}
			} 
			println!("");
		}   
		
		
}
