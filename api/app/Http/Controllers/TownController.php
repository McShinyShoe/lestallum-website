<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

use App\Models\Town;

class TownController extends Controller
{
    public function hello()
    {
        $town = Town::first();
        return make_response(null, 'Hello from ' . $town->name);
    }

    public function get()
    {
        $town = Town::first();
        return make_response($town, "Town retrieved");
    }

    public function update(Request $request)
    {
        $town = Town::firstOrFail();
        $town->update($request->only(['name', 'nation', 'mayor']));
        return make_response($town, "Town updated");
    }
}
