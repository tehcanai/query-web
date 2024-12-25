"use client";

import { useState } from "react";

export default function App() {
  const [submitted, setSubmitted] = useState(false);

  // @ts-expect-error Form type weirdness
  const onSubmit = async (formData) => {
    setSubmitted(true);

    const form = new FormData();

    form.append("query", formData.get("query"));

    try {
      await fetch("http://localhost:8000/query", {
        method: "POST",
        mode: "no-cors",
        body: form,
      })
        .then(async (response) => console.log(await response.text()))
        .catch((error) => {
          console.error("Error:", error);
        })
        .finally(() => setSubmitted(false));
    } catch (e) {
      console.log(e);
    }
  };

  return (
    <div className="bg-gray-800 grid grid-rows-[20px_1fr_20px] items-start justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
      <p className="text-3xl font-bold">CSV Query</p>
      <form action={onSubmit} className="w-1/2">
        <div className="w-full flex flex-row space-x-4">
          <input
            name="query"
            className="bg-gray-800 rounded-md w-full h-[45px] px-4 border border-slate-500 focus:outline:none"
          ></input>
          <button
            disabled={submitted}
            type="submit"
            className="font-bold text-xl rounded-md h-[45px] w-[45px] border border-slate-500"
          >{`>`}</button>
        </div>
      </form>
    </div>
  );
}
