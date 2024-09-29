pub mod onesies {
    pub fn count(i: u32) -> u32 {
        let mut a = i;
        let mut count = 0;
        while a > 0 {
            if (a & 0x1) == 1 {
                count+=1;
            }
            a >>= 1;
        }
        count
    }

}

#[cfg(test)]
mod tests {
    use super::onesies as onesies;

    #[test]
    fn it_works() {
        let result = onesies::count(1);
        assert_eq!(result, 1);
    }
    
    #[test]
    fn it_fails() {
        let result = onesies::count(2);
        assert_ne!(result, 2);
    }
}
