export default function Page({ params }: { params: { trainerId: string } }) {
    return (
        <div className="py-20 flex flex-col bg-slate-500 items-center px-10">
            <div className="border border-slate-500 border-b-white border-t-white w-full h-[200px] flex flex-col items-center">
                <div className="h-full bg-slate-500 w-[200px] flex justify-center -translate-y-14">
                    <img
                        src="https://picsum.photos/200/300"
                        alt="placeholder"
                        className="h-full w-9/12 object-cover"
                    />
                </div>
                <div className="-translate-y-5 bg-slate-500 px-4 flex flex-col items-center">
                    <div className="text-4xl"> This Is A Name</div>
                    <div className="text-xl"> Location </div>
                </div>
            </div>
        </div>
    );
}
