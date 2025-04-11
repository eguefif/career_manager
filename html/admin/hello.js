export function hello() {
    console.log("Hello");
}

function getHomeContent() {
        return `
    <section id="who-i-am-section">
    <button id="buildButton" type="submit" href="" class="button">Build website</button>
        <h1>I am Emmanuel Guefif</h1>
        <div class="who-i-am-container">
            <img src="./images/emmanuel.jpeg" alt="Your Picture" class="who-i-am-img">
            <div class="who-i-am-text">
                <p>
                    Lifelong learner, I made my first program when I was sixteen. It was a GCD calculator implementing an algorithm I had learned at school. In the first part of my adult life, I studied sociology and then became a teacher to share my passion for learning and nurture my student's curiosity. After meeting one of my student's father, I realized that talking about computers made me feel very good, and I decided to turn what was a passion into a profession. I now work as a full stack developper and learn everything I can about architecture.
            </p>
            </div>
        </div>
    </section>
    `
}
