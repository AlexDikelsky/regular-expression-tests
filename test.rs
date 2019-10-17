
fn combination(e :Vec<String>, usize k, String accumulated){

	// 1. stop
	if(e.length() < k) {
		return;
    }
	
	// 2. add each element in e to accumulated
	if(k == 1) {
		for s in e.chars() {
			print(accumulated+s);
        }
    }
	
	// 3. add all elements in e to accumulated
	else if(e.length() == k){
		for s in e.chars() {
			accumulated+=s;
        }
		print(accumulated);
	}
	
	// 4. for each element, call combination
	else if(e.size() > k) {
        let mut i = 0;
        while i < e.length() {
			combination(e[i+1..e.length()], k-1, accumulated+e[i]);
            i += 1;
        }
    }
}

