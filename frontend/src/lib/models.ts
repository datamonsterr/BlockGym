export class GymClass {
    flag: number;
    id: number;
    name: string;
    location: string;
    price: number;
    status: string;

    constructor(
        id: number,
        name: string,
        age: number,
        location: string,
        flag: number
    ) {
        this.flag = flag;
        this.id = id;
        this.name = name;
        this.price = age;
        this.location = location;
        if (flag === 0) {
            this.status = "Available";
        } else {
            this.status = "Unavailable";
        }
    }
}

export interface GymData {
    flag: number;
    company: string;
    customer: string;
    trainer: string;
    name: string;
    review:string;
    info: string;
    price: number;
    gym_class_pubkey: string;
}

export interface UserData {
    flag: number;
    owner: string;
    phone: string;
    name: string;
    email: string;
    location:string;
    info: string;
    age: number;
    gender: string;
    user_acc_pubkey: number;
    role: string;
}

