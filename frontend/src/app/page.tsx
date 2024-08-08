import TrainerItem from "@/components/custom/trainer-item";
import { Trainer } from "@/lib/models";

// Todo: Implement LoadGymClasses
function LoadGymClasses(): Trainer[] {
    let gymClasses: Trainer[] = [];
    for (let i = 0; i < 5; i++) {
        gymClasses.push(
            new Trainer(
                i,
                "Trainer " + i,
                20 + i,
                i % 2 === 0,
                "Location " + i,
                "Overview " + i
            )
        );
    }
    return gymClasses;
}

export default function Home() {
    let gymClasses = LoadGymClasses();
    return (
        <main className="grid grid-flow-row grid-cols-4 gap-10">
            {gymClasses.map((item) => {
                return TrainerItem(item);
            })}
        </main>
    );
}
