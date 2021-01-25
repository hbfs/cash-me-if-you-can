use std::collections::HashMap;
use std::collections::hash_map::Entry;
// use std::sync::Mutex;

static PRECISION: i64 = 10000;

pub struct Client {
	id: u16,
	total: i64,
	available: i64,
	held: i64,
	locked: bool,
	// mutex: Mutex<u8>,
}

impl Client {
	pub fn new(id: u16) -> Client {
		return Client {
			id: id,
			total: 0,
			available: 0,
			held: 0,
			locked: false,
			// mutex: Mutex::new(0),
		};
	}
}

pub enum TxType {
	DEPOSIT,
	WITHDRAWAL,
	DISPUTE,
	RESOLVE,
	CHARGEBACK,
}

pub enum TxStatus {
	NORMAL,
	DISPUTED,
	RESOLVED,
	CHARGEBACK,
}

pub struct Tx {
	id: u32,
	txtype: TxType,
	client_id: u16,
	amount: i64,
	status: TxStatus,
	// mutex: Mutex<u8>,
}

impl Tx {
	pub fn new(txtype: TxType, cid: u16, tid: u32, amount: i64) -> Tx {
		return Tx {
			id: tid,
			txtype: txtype,
			client_id: cid,
			amount: amount,
			status: TxStatus::NORMAL,
			// mutex: Mutex::new(0),
		};
	}
}

pub fn deposit(t: &Tx, c: &mut Client) {
	c.available +=  t.amount;
	c.total += t.amount;
}

pub fn withdraw(t: &Tx, c: &mut Client ) {
	if c.available >= t.amount {
		c.available -=  t.amount;
		c.total -= t.amount;
	}
	else {
		// log insufficient funds
		eprintln!("withdrawal insufficient funds: tx id: {}, client_id: {}", t.id, c.id);
	}	
}

pub fn dispute(t: &Tx, c: &Client) {
	/*
	availbe decrease by amount disputed
	held increase by amount disputed
	no amount, references amount by tx id
	if tx does not exists, assume partner error, no change
	*/
	// if t == None { return; } 

	

	
	
}

pub fn resolve(t: &Tx, c: &mut Client) {
	/*
	held funds decrease by amount disputed
	disputed
	if tx does not exists, or not disputed, assume partner error, no change
	*/
	// if (t == None || t.status != TxStatus::DISPUTED ) { return; }
		
}

pub fn chargeback(t: &Tx, c: &mut Client) {
	/*
	held funds decrease by amount disputed
	total funds decrease by amount disputed
	freeze account
	if tx does not exists, or not disputed, assume partner error, no change
	*/
	// if (t == None || t.status != TxStatus::DISPUTED ) { return; } 

	c.locked = true;

	
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn test_deposit() {
        assert_eq!(2 + 2, 4);
    }
}


pub fn ProcessTx(t: Tx, CLIENTS: &mut HashMap<u16, Client>, TXS: &mut HashMap<u32, Tx>) {
	let mut c: &mut Client = CLIENTS.entry(t.client_id).or_insert(Client::new(t.client_id));

	let t_id = t.id;
	let t_cid = t.client_id;

	let mut tx: &mut Tx = TXS.entry(t.id).or_insert(t);

	if t_cid != tx.client_id {
		// log tx doesn't belong to this client
		eprintln!("tx client mismatch tx id: {}, tx client_id: {}, client_id: {} mismatch", 
			t_id, t_cid, tx.client_id
		);
		return;
	}

	match tx.txtype {
		TxType::DEPOSIT => { deposit(&tx, &mut c); },
		TxType::WITHDRAWAL => { withdraw(&tx, &mut c); },
		TxType::DISPUTE => { dispute(&mut tx, &mut c); },
		TxType::RESOLVE => { resolve(&mut tx, &mut c); },
		TxType::CHARGEBACK => { chargeback(&mut tx, &mut c); },
		_ => return,
	}
}

pub fn PrintBalances(CLIENTS: &HashMap<u16, Client>) {
	println!("client, available, held, total, locked");
	
	for (key, value) in CLIENTS {
        println!(
        	"{}, {}, {}, {}, {}", 
        	key, value.available, value.held, value.total, value.locked
        );
    }
}

fn main() {
	let mut CLIENTS: HashMap<u16, Client> = HashMap::new();
	let mut TXS: HashMap<u32, Tx> = HashMap::new();

	// read CSV




/*
	let tx1 = Tx::new(TxType::DEPOSIT, 1, 1, 10);
	// let mut t: & Tx = TXS.entry(tx1.id).or_insert(tx1);
	TXS.insert(tx1.id, tx1);
	let mut t = TXS.get(&(1 as u32)).unwrap();
	// let Entry::Occupied(t) = TXS.entry(1 as u32);
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	let tx2 = Tx::new(TxType::DEPOSIT, 2, 2, 20);
	TXS.insert(tx2.id, tx2);
	// t = TXS.entry(tx2.id).or_insert(tx2);
	t = TXS.get(&(2 as u32)).unwrap();
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	let tx3 = Tx::new(TxType::DEPOSIT, 1, 3, 20);
	TXS.insert(tx3.id, tx3);
	// t = TXS.entry(tx3.id).or_insert(tx3);
	let Some(&t) = TXS.get(3);
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	let tx4 = Tx::new(TxType::WITHDRAWAL, 1, 4, 15);
	TXS.insert(tx4.id, tx4);
	// t = TXS.entry(tx4.id).or_insert(tx4);
	let Some(&t) = TXS.get(4);
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	let tx5 = Tx::new(TxType::WITHDRAWAL, 2, 5, 30);
	TXS.insert(tx5.id, tx5);	
	// t = TXS.entry(tx5.id).or_insert(tx5);
	let Some(&t) = TXS.get(5);
	ProcessTx(&t, &mut CLIENTS, &mut TXS);




	let mut tx;
	let mut tid;
	let mut t: &Tx;


	tx = Tx::new(TxType::DEPOSIT, 1, 1, 10);
	tid = tx.id;
	TXS.insert(tx.id, tx);
	t = TXS.get(&tid).unwrap();
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	tx = Tx::new(TxType::DEPOSIT, 2, 2, 20);
	tid = tx.id;
	TXS.insert(tx.id, tx);
	t = TXS.get(&tid).unwrap();
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	tx = Tx::new(TxType::DEPOSIT, 1, 3, 20);
	tid = tx.id;
	TXS.insert(tx.id, tx);
	t = TXS.get(&tid).unwrap();
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	tx = Tx::new(TxType::WITHDRAWAL, 1, 4, 15);
	tid = tx.id;
	TXS.insert(tx.id, tx);
	t = TXS.get(&tid).unwrap();
	ProcessTx(&t, &mut CLIENTS, &mut TXS);

	tx = Tx::new(TxType::WITHDRAWAL, 2, 5, 30);
	tid = tx.id;
	TXS.insert(tx.id, tx);
	t = TXS.get(&tid).unwrap();
	ProcessTx(&t, &mut CLIENTS, &mut TXS);
*/

	// add to queue, process queue

	let tx1 = Tx::new(TxType::DEPOSIT, 1, 1, 10);
	ProcessTx(tx1, &mut CLIENTS, &mut TXS);

	let tx2 = Tx::new(TxType::DEPOSIT, 2, 2, 20);
	ProcessTx(tx2, &mut CLIENTS, &mut TXS);

	let tx3 = Tx::new(TxType::DEPOSIT, 1, 3, 20);
	ProcessTx(tx3, &mut CLIENTS, &mut TXS);

	let tx4 = Tx::new(TxType::WITHDRAWAL, 1, 4, 15);
	ProcessTx(tx4, &mut CLIENTS, &mut TXS);

	let tx5 = Tx::new(TxType::WITHDRAWAL, 2, 5, 30);
	ProcessTx(tx5, &mut CLIENTS, &mut TXS);

	// Print balances

	PrintBalances(&CLIENTS);

/*
input
type, client, tx, amount
deposit, 1, 1, 1.0
deposit, 2, 2, 2.0
deposit, 1, 3, 2.0
withdrawal, 1, 4, 1.5
withdrawal, 2, 5, 3.0

output
client,available,held,total, 

1,1.5,0,1.5,false
2,2,0,2,false
*/





    
}
