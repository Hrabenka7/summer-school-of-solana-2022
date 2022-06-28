mod data_types;
mod flow_control;
mod functions;
mod ownership;
mod shadowing;
mod string_literal;
mod string_obj;
mod structure;
mod variable;
mod result;
mod option;
mod option_2;
mod enum_test;
mod trait_inheritance;
mod person_impl;
mod person_impl2;


fn main() {
    data_types::print_data_types();
    string_literal::print_string_literal();
    string_obj::print_string_obj();
    variable::print_string_obj();
    shadowing::print_string_obj();
    //println!("{}", functions::fn_hello());
    flow_control::flow_control();
    flow_control::ternal_like();
    ownership::ownership();
    result::result();
    option:: option();
    option_2:: option_2();
    enum_test:: enum_test();
    trait_inheritance::trait_inheritance();
    person_impl::person_impl();
    person_impl2::person_impl2();
}
