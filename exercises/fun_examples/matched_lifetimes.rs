fn test<'a: 'b, 'b>(v: &'b mut (&'a i32, &'a i32)) {
    let _ = v.1;
}

fn main() {
    let mut tuple = (&1, &2);
    {
        test(&mut tuple)
    }
    {
        test(&mut tuple)
    }
}
