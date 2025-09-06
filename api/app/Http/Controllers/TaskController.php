<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Storage;

class TaskController extends Controller
{
    public function index() {
        $tasks = Storage::disk('local')->directories('task');
        $tasks = array_map(fn($dir) => basename($dir), $tasks);

        return make_response($tasks, 'Success', 200, true, null);
    }
    public function show($id) {
        $taskDir = "task/{$id}";
        $contentPath = "task/{$id}/content.mdx";
        $rewardsPath = "task/{$id}/rewards.json";
        $playersPath = "task/{$id}/players.json";
        $tagsPath = "task/{$id}/tags.json";

        if (!(Storage::exists("{$taskDir}"))) {
            return make_response(null, 'Task not found.', Response::HTTP_NOT_FOUND, false, null);
        }

        $content = base64_encode(Storage::disk('local')->get($contentPath));
        $rewards = json_decode(Storage::disk('local')->get($rewardsPath), true);
        $players = json_decode(Storage::disk('local')->get($playersPath), true);
        $tags = json_decode(Storage::disk('local')->get($tagsPath), true);

        return make_response([
            'content' => $content,
            'rewards' => $rewards,
            'players' => $players,
            'tags' => $tags,
        ], 'Success');
    }

    public function store(Request $request, $id)
    {
        $request->validate([
            'content' => 'required|string',
            'rewards' => 'required|array',
            'rewards.*' => 'string',
            'tags' => 'required|array',
            'tags.*' => 'string'
        ]);

        $taskDir = "task/{$id}";

        if (Storage::disk('local')->exists("{$taskDir}")) {
            return make_response(null, 'Task already exists.', Response::HTTP_CONFLICT, false, null);
        }

        $content = base64_decode($request->content, true);
        if ($content === false) {
            return make_response(null, 'Invalid base64 content.', Response::HTTP_BAD_REQUEST, false, null);
        }

        Storage::disk('local')->put("{$taskDir}/content.mdx", $content);
        Storage::disk('local')->put("{$taskDir}/players.json", json_encode([]));
        Storage::disk('local')->put("{$taskDir}/rewards.json", json_encode($request->rewards));
        Storage::disk('local')->put("{$taskDir}/tags.json", json_encode($request->tags));

        return make_response(null, 'Task created.', Response::HTTP_CREATED);
    }

    public function update(Request $request, $id)
    {
        $request->validate([
            'content' => 'required|string',
            'rewards' => 'required|array',
            'rewards.*' => 'string',
            'tags' => 'required|array',
            'tags.*' => 'string'
        ]);

        $taskDir = "task/{$id}";
        $filePath = "{$taskDir}/content.mdx";

        if (!Storage::disk('local')->exists($filePath)) {
            return make_response(null, 'Task not found.', Response::HTTP_NOT_FOUND, false, null);
        }

        $content = base64_decode($request->content, true);
        if ($content === false) {
            return make_response(null, 'Invalid base64 content.', Response::HTTP_BAD_REQUEST, false, null);
        }

        Storage::disk('local')->put($filePath, $content);
        Storage::disk('local')->put("{$taskDir}/rewards.json", json_encode($request->rewards));
        Storage::disk('local')->put("{$taskDir}/tags.json", json_encode($request->tags));

        return make_response(null, 'Task updated.');
    }

    public function destroy($id)
    {
        $taskDir = "task/{$id}";

        if (!Storage::disk('local')->exists("{$taskDir}")) {
            return make_response(null, 'Task not found.', Response::HTTP_NOT_FOUND, false, null);
        }

        Storage::disk('local')->deleteDirectory($taskDir);

        return make_response(null, 'Task deleted.');
    }


    public function showFile($id, $filename) {
        $filePath = "task/{$id}/files/{$filename}";

        if (!Storage::exists($filePath)) {
            return make_response(null, 'File not found', Response::HTTP_NOT_FOUND, false, null);
        }

        $extension = pathinfo($filename, PATHINFO_EXTENSION);
        $mimeType = match($extension) {
            'md', 'mdx' => 'text/markdown',
            'txt', 'json', 'csv', 'log' => 'text/plain',
            'pdf' => 'application/pdf',
            'jpg', 'jpeg' => 'image/jpeg',
            'png' => 'image/png',
            'gif' => 'image/gif',
            'svg' => 'image/svg+xml',
            'webp' => 'image/webp',
            default => 'application/octet-stream',
        };

        $contents = Storage::disk('local')->get($filePath);

        return response($contents, 200)->header('Content-Type', $mimeType);
    }




public function join(Request $request, $id)
{
    $user = $request->user();  // Sanctum authenticated user
    $mcName = $user->mc_name;

    $playersPath = "task/{$id}/players.json";

    // Load existing players
    $players = Storage::exists($playersPath)
        ? json_decode(Storage::get($playersPath), true)
        : [];

    if (!is_array($players)) $players = [];

    // Add user if not already in the list
    if (!in_array($mcName, $players)) {
        $players[] = $mcName;
        Storage::put($playersPath, json_encode($players, JSON_PRETTY_PRINT));
    }

    return response()->json([
        'success' => true,
        'message' => "{$mcName} joined task {$id}",
        'players' => $players
    ]);
}

public function leave(Request $request, $id)
{
    $user = $request->user();  // Sanctum authenticated user
    $mcName = $user->mc_name;

    $playersPath = "task/{$id}/players.json";

    $players = Storage::exists($playersPath)
        ? json_decode(Storage::get($playersPath), true)
        : [];

    if (!is_array($players)) $players = [];

    // Remove user if present
    $players = array_values(array_diff($players, [$mcName]));

    Storage::put($playersPath, json_encode($players, JSON_PRETTY_PRINT));

    return response()->json([
        'success' => true,
        'message' => "{$mcName} left task {$id}",
        'players' => $players
    ]);
}





}
