import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";

class GymClass {
  title: string;
  constructor(title: string) {
    this.title = title;
  }
}

// Todo: Implement LoadGymClasses
function LoadGymClasses() {
  return [
    {
      title: "Card 1",
    },
    {
      title: "Card 2",
    },
    {
      title: "Card 3",
    },
  ];
}

export default function Home() {
  let gymClasses = LoadGymClasses();
  return (
    <main>
      {gymClasses.map((card) => (
        <div>{card.title}</div>
      ))}
    </main>
  );
}
