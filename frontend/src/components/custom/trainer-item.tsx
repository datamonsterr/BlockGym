import { Trainer } from "@/lib/models";
import * as React from "react";

import { Button } from "@/components/ui/button";
import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Link } from "lucide-react";

export default function TrainerItem(trainer: Trainer) {
    return (
        <Card className="w-[350px]">
            <div className="w-full">
                <img
                    src="https://swequity.vn/wp-content/uploads/2019/12/bai-tap-leanbody-1024x683.png"
                    alt="placeholder"
                    className="w-full h-[300px] object-cover"
                />
            </div>
            <CardContent className="mt-2">
                <div className="text-3xl">{trainer.name}</div>
                <div className="mt-2">
                    <div>Location: {trainer.location}</div>
                    <div>Age: {trainer.age}</div>
                    <div>Gender: {trainer.gender ? "female" : "male"}</div>
                </div>
            </CardContent>
            <CardFooter className="flex justify-end">
                <a href={"/trainer/" + trainer.id}>
                    <Button>Book</Button>
                </a>
            </CardFooter>
        </Card>
    );
}
