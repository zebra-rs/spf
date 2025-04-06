use spf::*;

#[test]
pub fn repair() {
    let pc_inter1 = vec![
        Intersect {
            id: 1,
            p: true,
            q: false,
        },
        Intersect {
            id: 2,
            p: true,
            q: false,
        },
        Intersect {
            id: 3,
            p: true,
            q: true,
        },
        Intersect {
            id: 4,
            p: false,
            q: true,
        },
        Intersect {
            id: 5,
            p: false,
            q: true,
        },
    ];

    let pc_inter2 = vec![
        Intersect {
            id: 1,
            p: true,
            q: false,
        },
        Intersect {
            id: 2,
            p: true,
            q: false,
        },
        Intersect {
            id: 3,
            p: true,
            q: false,
        },
        Intersect {
            id: 4,
            p: false,
            q: true,
        },
        Intersect {
            id: 5,
            p: false,
            q: true,
        },
    ];

    let pc_inter3 = vec![
        Intersect {
            id: 1,
            p: true,
            q: false,
        },
        Intersect {
            id: 2,
            p: true,
            q: false,
        },
        Intersect {
            id: 3,
            p: true,
            q: false,
        },
        Intersect {
            id: 4,
            p: false,
            q: false,
        },
        Intersect {
            id: 5,
            p: false,
            q: true,
        },
    ];

    let pc_inter4 = vec![
        Intersect {
            id: 1,
            p: true,
            q: false,
        },
        Intersect {
            id: 2,
            p: true,
            q: false,
        },
        Intersect {
            id: 3,
            p: true,
            q: false,
        },
        Intersect {
            id: 4,
            p: false,
            q: false,
        },
        Intersect {
            id: 5,
            p: false,
            q: false,
        },
    ];

    let pc_inter5 = vec![
        Intersect {
            id: 1,
            p: true,
            q: false,
        },
        Intersect {
            id: 2,
            p: false,
            q: false,
        },
        Intersect {
            id: 3,
            p: false,
            q: false,
        },
        Intersect {
            id: 4,
            p: false,
            q: false,
        },
        Intersect {
            id: 5,
            p: false,
            q: true,
        },
    ];

    let pc_inter6 = vec![
        Intersect {
            id: 1,
            p: false,
            q: false,
        },
        Intersect {
            id: 2,
            p: false,
            q: false,
        },
        Intersect {
            id: 3,
            p: false,
            q: false,
        },
        Intersect {
            id: 4,
            p: false,
            q: false,
        },
        Intersect {
            id: 5,
            p: false,
            q: false,
        },
    ];

    let repair_list = make_repair_list(&pc_inter1, 0, 100);
    println!("{:?}", repair_list);
    let repair_list = make_repair_list(&pc_inter2, 0, 100);
    println!("{:?}", repair_list);
    let repair_list = make_repair_list(&pc_inter3, 0, 100);
    println!("{:?}", repair_list);
    let repair_list = make_repair_list(&pc_inter4, 0, 100);
    println!("{:?}", repair_list);
    let repair_list = make_repair_list(&pc_inter5, 0, 100);
    println!("{:?}", repair_list);
    let repair_list = make_repair_list(&pc_inter6, 0, 100);
    println!("{:?}", repair_list);
}
