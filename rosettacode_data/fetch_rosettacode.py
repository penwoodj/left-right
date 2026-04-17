#!/usr/bin/env python3
"""
Rosetta Code Data Extractor for Left-Right Language Design Research

Fetches task lists from Rosetta Code and categorizes by semantic domain.
Counts examples for 27 target languages and provides URL patterns.
"""

import re
import json
import urllib.request
from urllib.parse import quote
import os
import time

# Configuration
BASE_URL = "https://rosettacode.org"
OUTPUT_DIR = "/home/jon/code/left-right/rosettacode_data"

# 27 Target Languages for Left-Right research
TARGET_LANGUAGES = [
    "Python", "JavaScript", "ALGOL", "Ruby", "Haskell", "OCaml",
    "Clojure", "CoffeeScript", "Groovy", "Wren", "Elixir", "Rust",
    "Java", "C", "C++", "Go", "Swift", "Kotlin", "Scala",
    "TypeScript", "Julia", "Racket", "F#", "Scheme", "Common Lisp",
    "Erlang"
]

# Semantic Domain Mapping (Rosetta Code categories → Semantic domains)
SEMANTIC_DOMAINS = {
    "Arithmetic": ["math", "numerical", "calculation", "arithmetic", "factorial", "fibonacci"],
    "Data Structures": ["data", "structures", "collections", "array", "list", "hash", "tree", "stack", "queue"],
    "Classic CS problems": ["algorithms", "puzzles", "classical", "sort", "search", "optimization"],
    "Text Processing": ["text", "string", "parsing", "format", "json", "xml"],
    "Graphics": ["graphics", "image", "visual", "animation", "bitmap", "draw"],
    "Date and time": ["time", "date", "calendar"],
    "Concurrency": ["concurrent", "parallel", "thread", "async", "lock", "semaphore"],
    "Encryption": ["encryption", "cipher", "crypto", "hash", "password"],
    "File I/O": ["file", "input", "output", "directory", "read", "write"],
    "System": ["system", "process", "shell", "command", "environment"],
    "Networking": ["network", "web", "http", "socket", "tcp", "udp"],
    "Database": ["database", "sql", "db", "storage"],
    "GUI": ["gui", "window", "interface", "dialog", "button"],
    "Games": ["game", "play", "simulation", "puzzle", "chess", "tic-tac-toe"],
    "Math": ["geometry", "trigonometry", "statistics", "calculus", "matrix"]
}

def fetch_page(url, max_retries=3):
    """Fetch a web page with retries."""
    for attempt in range(max_retries):
        try:
            req = urllib.request.Request(
                url,
                headers={'User-Agent': 'Left-Right-Research/1.0'}
            )
            with urllib.request.urlopen(req, timeout=30) as response:
                return response.read().decode('utf-8')
        except Exception as e:
            if attempt < max_retries - 1:
                time.sleep(1)
                continue
            print(f"Failed to fetch {url}: {e}")
            return None

def parse_task_list(html_content):
    """Extract task names from category page HTML using regex."""
    if not html_content:
        return []
    
    tasks = []
    
    # Find all wiki links that look like tasks
    # Pattern: <a href="/wiki/Task_Name">Task Name</a>
    # Exclude Category:, Help:, Rosetta_Code: links
    pattern = r'<a\s+href="(/wiki/[^"]+)"[^>]*>([^<]+)</a>'
    matches = re.findall(pattern, html_content)
    
    for href, text in matches:
        # Filter out non-task links
        if (href.startswith('/wiki/') and 
            not '/wiki/Category:' in href and
            not '/wiki/Help:' in href and
            not '/wiki/Rosetta_Code' in href and
            not '/wiki/Talk:' in href and
            not ':' in text and
            text.strip()):
            task_name = text.strip()
            if len(task_name) > 2 and len(task_name) < 100:
                tasks.append(task_name)
    
    # Remove duplicates while preserving order
    seen = set()
    unique_tasks = []
    for task in tasks:
        if task not in seen:
            seen.add(task)
            unique_tasks.append(task)
    
    return unique_tasks

def get_total_count(html_content):
    """Extract total task count from category page."""
    if not html_content:
        return 0
    
    # Pattern: "The following 200 pages are in this category, out of 1,349 total."
    match = re.search(r'out of ([0-9,]+) total', html_content)
    if match:
        return int(match.group(1).replace(',', ''))
    return 0

def categorize_task(task_name):
    """Categorize a task by semantic domain based on keywords."""
    task_lower = task_name.lower()
    
    for domain, keywords in SEMANTIC_DOMAINS.items():
        for keyword in keywords:
            if keyword.lower() in task_lower:
                return domain
    
    return "Other"

def fetch_language_tasks(language_name):
    """Fetch all tasks for a specific language."""
    print(f"Fetching tasks for {language_name}...")
    
    # Try multiple URL patterns
    possible_urls = [
        f"{BASE_URL}/wiki/{quote(language_name)}",
        f"{BASE_URL}/wiki/{quote(language_name.replace(' ', '_'))}",
        f"{BASE_URL}/wiki/Category:{quote(language_name)}",
    ]
    
    for url in possible_urls:
        html = fetch_page(url)
        if html:
            tasks = parse_task_list(html)
            if tasks and len(tasks) > 5:  # Must have meaningful content
                print(f"  Found {len(tasks)} tasks from {url}")
                return tasks
    
    print(f"  No tasks found for {language_name}")
    return []

def build_dataset():
    """Build the complete Rosetta Code dataset."""
    print("Building Rosetta Code dataset for Left-Right research...")
    
    # Initialize master dataset
    dataset = {
        "metadata": {
            "source": "Rosetta Code",
            "target_languages": len(TARGET_LANGUAGES),
            "extracted_at": time.strftime("%Y-%m-%d %H:%M:%S")
        },
        "categories": {},
        "languages": {},
        "semantic_domains": list(SEMANTIC_DOMAINS.keys())
    }
    
    # Step 1: Fetch main categories
    print("\n=== Step 1: Fetching main categories ===")
    categories = ["Arithmetic", "Data Structures", "Classic CS problems", "Sorting", "Encryption"]
    
    for category in categories:
        url = f"{BASE_URL}/wiki/Category:{quote(category)}"
        print(f"Fetching {category}...")
        html = fetch_page(url)
        
        if html:
            tasks = parse_task_list(html)
            total = get_total_count(html)
            dataset["categories"][category] = {
                "url": url,
                "task_count": total,
                "tasks_sample": tasks[:15],  # Sample first 15
                "domain": categorize_task(category)
            }
            print(f"  {category}: {total} total tasks, {len(tasks)} extracted")
    
    # Step 2: Fetch tasks for each target language
    print("\n=== Step 2: Fetching tasks per language ===")
    
    for lang in TARGET_LANGUAGES:
        tasks = fetch_language_tasks(lang)
        
        if tasks:
            # Categorize tasks by semantic domain
            categorized = {}
            for task in tasks:
                domain = categorize_task(task)
                if domain not in categorized:
                    categorized[domain] = []
                categorized[domain].append(task)
            
            dataset["languages"][lang] = {
                "total_tasks": len(tasks),
                "by_domain": {k: len(v) for k, v in categorized.items()},
                "task_list_sample": tasks[:5]  # Sample first 5
            }
        else:
            dataset["languages"][lang] = {
                "total_tasks": 0,
                "by_domain": {},
                "task_list_sample": []
            }
        
        # Be polite to the server
        time.sleep(0.3)
    
    # Step 3: Generate URL patterns
    print("\n=== Step 3: Generating URL patterns ===")
    dataset["url_patterns"] = {
        "category_page": "{base}/wiki/Category:{category}",
        "language_page": "{base}/wiki/{language}",
        "task_solutions": "{base}/wiki/{language}/{task}",
        "examples": [
            {"description": "Python factorial", "url": f"{BASE_URL}/wiki/Python/Factorial"},
            {"description": "JavaScript arrays", "url": f"{BASE_URL}/wiki/JavaScript/Array_concatenation"},
            {"description": "Haskell fibonacci", "url": f"{BASE_URL}/wiki/Haskell/Fibonacci_sequence"},
            {"description": "ALGOL tasks", "url": f"{BASE_URL}/wiki/ALGOL"}
        ]
    }
    
    return dataset

def save_dataset(dataset):
    """Save dataset to JSON files."""
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)
    
    # Save master dataset
    master_file = os.path.join(OUTPUT_DIR, "rosettacode_master.json")
    with open(master_file, 'w', encoding='utf-8') as f:
        json.dump(dataset, f, indent=2, ensure_ascii=False)
    print(f"\nSaved master dataset to {master_file}")
    
    # Save per-language data
    for lang, data in dataset["languages"].items():
        lang_file = os.path.join(OUTPUT_DIR, f"{lang.replace(' ', '_')}.json")
        with open(lang_file, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
    
    print(f"Saved per-language files to {OUTPUT_DIR}")
    
    # Save summary statistics
    summary = {
        "total_languages": len(TARGET_LANGUAGES),
        "languages_with_tasks": sum(1 for l in dataset["languages"].values() if l["total_tasks"] > 0),
        "total_tasks_across_all_languages": sum(l["total_tasks"] for l in dataset["languages"].values()),
        "top_domains": {}
    }
    
    # Aggregate by domain
    for lang, data in dataset["languages"].items():
        for domain, count in data["by_domain"].items():
            if domain not in summary["top_domains"]:
                summary["top_domains"][domain] = 0
            summary["top_domains"][domain] += count
    
    summary_file = os.path.join(OUTPUT_DIR, "summary.json")
    with open(summary_file, 'w', encoding='utf-8') as f:
        json.dump(summary, f, indent=2, ensure_ascii=False)
    print(f"Saved summary to {summary_file}")

def main():
    print("=" * 60)
    print("Rosetta Code Data Extractor")
    print("Left-Right Language Design Research")
    print("=" * 60)
    
    dataset = build_dataset()
    save_dataset(dataset)
    
    print("\n" + "=" * 60)
    print("Extraction complete!")
    print(f"Output directory: {OUTPUT_DIR}")
    print("=" * 60)

if __name__ == "__main__":
    main()
