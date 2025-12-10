use adder::add;

fn it_adds(){
    let result = add(2,2);
    assert_eq!(4);
}

// to tes--> cargo test --test integration_test