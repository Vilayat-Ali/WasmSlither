export function updateGithubStarCount() {
  const starCountCard = document.querySelector("#github-stars");
  fetch(`https://api.github.com/repos/vilayat-ali/wasmslither`)
    .then((response) => response.json())
    .then((data) => {
      starCountCard.innerHTML = data.stargazers_count;
    })
    .catch((error) => console.error(error));
}
