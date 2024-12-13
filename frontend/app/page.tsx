export default function App() {
  return (
    <div className="grid grid-rows-[20px_1fr_20px] items-start justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <p className="text-3xl font-bold">CSV Query</p>
      <input
        name="query"
        className="bg-slate-800 rounded-md h-[40px] w-1/4 px-4 border border-slate-500"
      ></input>
    </div>
  );
}
