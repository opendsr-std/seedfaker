use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let titles = [
        "Software Engineer",
        "Product Manager",
        "Data Analyst",
        "VP Engineering",
        "CFO",
        "HR Director",
        "Sales Manager",
        "DevOps Engineer",
        "QA Lead",
        "CTO",
        "Marketing Director",
        "Operations Manager",
        "Legal Counsel",
        "Compliance Officer",
        "Security Analyst",
        "UX Designer",
        "Data Scientist",
        "Account Executive",
        "Support Engineer",
        "Infrastructure Lead",
        "Frontend Developer",
        "Backend Developer",
        "Full Stack Developer",
        "Solutions Architect",
        "Technical Writer",
        "Scrum Master",
    ];
    buf.push_str(titles[ctx.rng.urange(0, titles.len() - 1)]);
}
