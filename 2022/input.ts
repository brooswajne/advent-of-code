import { TextLineStream } from "https://deno.land/std@0.167.0/streams/text_line_stream.ts";

/** Auto-detects the directory of the current challenge, just for fun, and returns the URL to its input file. */
function getInputPath() {
  const { stack } = new Error();
  if (stack == null) throw new Error("Uhhh this shouldn't happen");

  const filesInCallStack = stack.split("\n")
    .map((line) => line.match(/(file:\/\/.*?):/))
    .filter((match): match is RegExpMatchArray => match != null)
    .map((match) => match[1]);

  const solutionFile = filesInCallStack
    .find((file) => file.endsWith("/solution.ts"));
  if (solutionFile == null) {
    const message = "Unable to find solution file in call-stack" +
      ` (earliest file found: ${filesInCallStack.at(-1)})`;
    throw new Error(message);
  }

  return new URL("input", solutionFile);
}

export async function* getInputLines(path = getInputPath()) {
  const { body } = await fetch(path).catch(function handle(err) {
    throw new Error(`Unable to read input from ${path}`, { cause: err });
  });
  // Should not happen in this case, only relevant for web requests with CORS restrictions
  if (body == null) throw new Error("Unable to access response body");

  yield* body
    .pipeThrough(new TextDecoderStream())
    .pipeThrough(new TextLineStream());
}
