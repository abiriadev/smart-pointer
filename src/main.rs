mod cons_list;
mod drop;
mod memory_leak;
mod my_box;
mod tree;

fn main() {
    drop::drop_main();
    my_box::my_box_main();

    cons_list::list::list_main();
    cons_list::rc_list::list_main();
    cons_list::rc_refcell_list::list_main();

    memory_leak::main();
    tree::main();
}
