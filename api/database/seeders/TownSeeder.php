<?php

namespace Database\Seeders;

use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Database\Seeder;

use App\Models\Town;

class TownSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        Town::firstOrCreate(
            ['id' => 1],
            [
                'name'   => '-',
                'nation' => '-',
                'mayor'  => '-',
            ]
        );
    }
}
