# European Cities Tour Planner (Traveling Salesman Problem)

<!--toc:start-->

- [European Cities Tour Planner (Traveling Salesman Problem)](#european-cities-tour-planner-traveling-salesman-problem)
  - [📌 Project Description](#📌-project-description)
  - [📥 Input Data](#📥-input-data)
  - [Instructions](#instructions)
  - [Results and comparisons](#results-and-comparisons)
    - [🧪 Test Results](#🧪-test-results)
    - [📊 Analysis](#📊-analysis)
      - [Dynamic Programming](#dynamic-programming)
      - [Genetic Algorithm (GA)](#genetic-algorithm-ga)
      - [Parallel Genetic Algorithm (GAP)](#parallel-genetic-algorithm-gap)
    - [Conclusion](#conclusion)
  - [Leftover TODO's](#leftover-todos)
  <!--toc:end-->

## 📌 Project Description

In the TSP the input is a list of cities and the cost of traveling between each city.
The goal of the salesman is to determine the **shortest possible route**.

In this project the following algorithms are implemented:

- **Dynamic Programming (Held-Karp)**
- **Genetic Algorithm**
- **Parallel Genetic Algorithm**

Rayon was used as concurrency since it has a thread pool and allows us to create several more task than we have available threads.

![Example](assets/basic_dp.png)

---

## 📥 Input Data

The list of cities and the distances between them (in kilometers) are provided in a `.txt` file that accompanies this project.

---

## Instructions

Run the project with:

`cargo run --release`

---

## Results and Comparisons

Each algorithm was tested on 3 problem sizes: 4, 19, and 100 cities. For the 100-city case, the GA and GAP were configured with different population sizes to evaluate the scalability and effectiveness of parallelism.

### 🧪 Test Results

| Cities | Algorithm | Configuration                  | Time      | Outcome                 |
| ------ | --------- | ------------------------------ | --------- | ----------------------- |
| 4      | DP        | N/A                            | 110 μs    | ✅ Optimal solution     |
| 19     | DP        | N/A                            | 629 ms    | ✅ Optimal solution     |
| 100    | DP        | N/A                            | ❌ OOM    | ❌ Out of memory        |
| 19     | GA        | pop=100, gens=100k, elitism=3  | 3780 ms   | ✅ Converged to optimal |
| 100    | GA        | pop=100, gens=100k, elitism=3  | 4578 ms   | ❌ Did not converge     |
| 100    | GA        | pop=1000, gens=100k, elitism=3 | 242555 ms | ❌ Did not converge     |
| 19     | GAP       | pop=100, gens=100k, elitism=3  | 5078 ms   | ✅ Converged to optimal |
| 100    | GAP       | pop=100, gens=100k, elitism=3  | 5569 ms   | ❌ Did not converge     |
| 100    | GAP       | pop=1000, gens=100k, elitism=3 | 111638 ms | ❌ Did not converge     |

---

### 📊 Analysis

#### Dynamic Programming

- **Fast and optimal** for small inputs (≤ 19 cities).
- **Out of memory** at 100 cities — factorial complexity becomes unmanageable.

#### Genetic Algorithm (GA)

- Effective on medium-sized problems (19 cities).
- Slower convergence at 100 cities with small populations.
- Increasing the population to 1000 improves solution space exploration but significantly increases computation time.

#### Parallel Genetic Algorithm (GAP)

- Adds parallelism (e.g., with Rayon) to speed up genetic operations.
- At **population=1000**, it nearly halves the execution time compared to GA (111s vs 242s).
- Shows **clear advantage** only with **larger populations and problem sizes** — parallelism overhead is too high for small tasks.

### Conclusion

- Use **brute-force** only for very small TSP instances (N ≤ 19).
- Use **GA** for medium-sized problems where a near-optimal solution is acceptable.
- Use **GAP** when dealing with **large populations or larger problem spaces** — this is where parallelism begins to outperform the sequential version.

---

## Leftover TODO's

- [ ] Do not sort population but rather utilize a min-max heap to store the elitism values.
- [ ] Add configuration for GA.
- [ ] Try out different algorithms for selection instead of roulette wheel selection (ex. Tournament selection).
- [ ] Add circle layout

---
