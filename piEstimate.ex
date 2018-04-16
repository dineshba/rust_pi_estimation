defmodule PiEstimation do

  def estimate_pi(total_points, total_threads) do
    now = Time.utc_now
    total_in_circle_points = compute_in_multiple_threads(total_points, total_threads)

    IO.inspect(4.0 * total_in_circle_points / total_points)
    IO.inspect("time: #{Time.diff(Time.utc_now, now, :microsecond) / 1000000}s")
  end

  defp compute_in_multiple_threads(total_points, total_threads) do
    Enum.each(0..(total_threads - 1), fn _ ->
      spawn(PiEstimation, :compute_in_single_thread, [
        Integer.floor_div(total_points, total_threads),
        self
      ])
    end)

    receive_values(total_threads)
  end

  def compute_in_single_thread(loop_count, sender) do
    in_circle_count =
      0..(loop_count - 1)
      |> Enum.map(fn _ ->
        a = :random.uniform() * 2 - 1
        b = :random.uniform() * 2 - 1
        a * a + b * b
      end)
      |> Enum.filter(fn a -> a <= 1 end)
      |> Enum.count()

    send(sender, in_circle_count)
  end

  defp receive_values(total_threads, acc \\ 0)
  defp receive_values(0, acc), do: acc

  defp receive_values(total_threads, acc) do
    receive do
      count -> receive_values(total_threads - 1, acc + count)
    end
  end
end
