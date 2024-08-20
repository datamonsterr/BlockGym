import { Button } from "@/components/ui/button";
export default function Page() {
    return (
        <div
            className="w-full h-[700px] bg-red-100 flex flex-col justify-end items-center"
            style={{
                backgroundImage: `url(https://www.thethaodaiviet.vn/upload/thoi-diem-dung-thuc-pham-bo-sung-cho-gymer.jpg?v=1.0.0)`,
                backgroundRepeat: "no-repeat",
                backgroundSize: "cover",
                WebkitBackgroundSize: "cover",
            }}
        >
            <div className="flex justify-between w-1/2 h-[200px]">
                <a href="/trainer">
                    <Button
                        variant="secondary"
                        className="px-10 py-10 text-3xl"
                    >
                        I am Trainer
                    </Button>
                </a>
                <a href="/view-gym-class">
                    <Button className="px-10 py-10 text-3xl">
                        I want to hire
                    </Button>
                </a>
            </div>
        </div>
    );
}
