export class Trainer {
    id: number;
    name: string;
    age: number;
    gender: boolean;
    location: string;
    overview: string;

    constructor(
        id: number,
        name: string,
        age: number,
        gender: boolean,
        location: string,
        overview: string
    ) {
        this.id = id;
        this.name = name;
        this.age = age;
        this.gender = gender;
        this.location = location;
        this.overview = overview;
    }
}
