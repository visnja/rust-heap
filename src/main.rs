use std::fmt;


struct HeapNode {
	value: String,
	key: String
}

impl fmt::Display for HeapNode {
	fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[Key: {}, Value {}]", self.key, self.value)
	}
	// add code here
}

struct Heap {
	data: Vec<HeapNode>,
}


impl Heap {
	pub fn parent(&mut self, i: usize) -> usize {
		i.wrapping_sub(1) / 2
	}
	fn left(&mut self, i: usize) -> usize {
		2 * i + 1
	}
	fn right(&mut self, i: usize) -> usize {
		2 * i + 2
	}

	pub fn len(&mut self) -> usize {
		self.data.len()
	}

	 fn has_left(&mut self, i: usize) -> bool {
		self.left(i) < self.len()
	}
	 fn has_right(&mut self, i: usize) -> bool {
		self.right(i) < self.len()
	}

	fn swap(&mut self, i: usize, j: usize) {
		self.data.swap(i,j)
	}
 	fn up_heap(&mut self, i: usize){
		let parent = self.parent(i);
		if i > 0 && self.data[i].key > self.data[parent].key {
			self.swap(i, parent);
			self.up_heap(parent);
		}
	}
	fn down_heap(&mut self, i: usize){
		if self.has_left(i){
			let left = self.left(i);
			let mut small_child = left;
			if self.has_right(i){
				let right = self.right(i);
				if self.data[right].key > self.data[left].key {
					small_child = right
				}
			}
			if self.data[small_child].key > self.data[i].key {
				self.swap(i,small_child);
				self.down_heap(small_child);
			}	
		}
	}
	pub fn add(&mut self, key: String, value: String) {
		self.data.push(HeapNode{key,value});
		let idx = self.len() - 1;
		self.up_heap(idx)
	}

	pub fn max(&mut self) -> Option<&HeapNode> {
		if self.len() == 0 {
			None
		}else{
			Some(&self.data[0])
		}
		
	}
	pub fn remove_max(&mut self) -> Option<HeapNode>{
		if self.len() == 0 {
			None
		}else{
			let idx = self.len() - 1;
			self.swap(0, idx);
			let item = self.data.pop().unwrap();
			self.down_heap(0);
			Some(item)
		}
	}
}



fn main() {
    let mut heap = Heap{data: Vec::new()};
    heap.add("b".to_string(),"world".to_string());
    heap.add("c".to_string(),"baba".to_string());
    heap.add("a".to_string(),"blacksheep".to_string());
    heap.add("d".to_string(),"hello".to_string());
    match heap.remove_max(){
    	None => println!("No item found"),
    	Some(item) =>  println!("Removing {}", item )
    }
    match heap.max(){
    	None => println!("No item found"),
    	Some(item) =>  println!("{}", item )
    }
    println!("The size is {}", heap.len() );
}
