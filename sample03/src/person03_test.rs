
//20200929
mod person03_test{
    use super::super::*;

    #[test]
    fn sample03_test_without_blake3hash(){
        let mut person1 =  Person03::new(String :: from("abc123"), 0, false);
        let rs1 = person1.without_blake3hash().unwrap();
        assert_eq!(true, rs1);
    } 

    #[test]
    fn sample03_test_use_blake3hash(){
        let mut person2 =  Person03::new(String :: from("goh-1724"), 0, false);
        let rs2 = person2.use_blake3hash().unwrap();
        assert_eq!(true, rs2);
    } 


    #[test]
    fn sample03_person_get_name4(){
        let person3 =  Person03::new(String :: from("alex"), 0, false);
        assert_eq!("alex", person3.get_name());
    } 

    #[test]
    fn sample03_person_get_age5(){
        let person4 =  Person03::new(String :: from("alex"), 13, false);
        assert_eq!(13, person4.get_age());
    } 

    #[test]
    fn sample03_person_can_walked6(){
        let person5 =  Person03::new(String :: from("alex"), 13, true);
        assert_eq!(true, person5.can_walked());
    } 

}
